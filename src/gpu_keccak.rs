use ocl::{Buffer, Context, Device, MemFlags, Platform, ProQue, Program, Queue};
static KERNEL_SRC: &'static str = include_str!("./kernels/keccak256.cl");
const WORK_SIZE: usize = 0x4000000;
pub fn calc_keccak_gpu(txt: String, gpu_device: u8) -> Result<Vec<u8>, ocl::Error> {
    let platform = Platform::default();
    let device = Device::by_idx_wrap(platform, gpu_device as usize)?;
    let context = Context::builder()
        .platform(platform)
        .devices(device.clone())
        .build()?;
    let program = Program::builder()
        .devices(device)
        .src(KERNEL_SRC)
        .build(&context)?;
    let queue = Queue::new(&context, device, None)?;

    let ocl_pq = ProQue::new(context, queue, program, Some(WORK_SIZE));

    let message = to_fixed_85(txt.as_bytes());
    let message_buffer = Buffer::builder()
        .queue(ocl_pq.queue().clone())
        .flags(MemFlags::new().read_only())
        .len(85)
        .copy_host_slice(&message)
        .build()?;

    let mut solution: Vec<u8> = vec![0; 256];
    let solution_buffer = Buffer::builder()
        .queue(ocl_pq.queue().clone())
        .flags(MemFlags::new().write_only())
        .len(32)
        .copy_host_slice(&solution)
        .build()?;

    // static inline void keccak256(uchar *digest, uchar const *message)
    let kern = ocl_pq
        .kernel_builder("keccak256")
        .arg_named("digest", None::<&Buffer<u8>>)
        .arg_named("message", None::<&Buffer<u8>>)
        .build()?;

    kern.set_arg("digest", &solution_buffer)?;
    kern.set_arg("message", &message_buffer)?;

    unsafe {
        kern.enq()?;
    }

    solution_buffer.read(&mut solution).enq()?;
    return Ok(solution);
}

fn to_fixed_85(bytes: &[u8]) -> [u8; 85] {
    let mut array = [0; 85];
    let bytes = &bytes[..array.len()];
    array.copy_from_slice(bytes);
    array
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_keccak_gpu() {
        let txt = "Hello World!".to_string();
        let result = calc_keccak_gpu(txt, 0).unwrap();
        assert_eq!(
            result,
            vec![
                0x6f, 0x2b, 0x2b, 0x2b, 0x2b, 0x2b, 0x2b, 0x2b, 0x2b, 0x2b, 0x2b, 0x2b, 0x2b, 0x2b,
                0x2b, 0x2b, 0x2b, 0x2b, 0x2b, 0x2b, 0x2b, 0x2b, 0x2b, 0x2b, 0x2b, 0x2b, 0x2b, 0x2b,
                0x2b, 0x2b, 0x2b, 0x2b
            ]
        );
    }
}
