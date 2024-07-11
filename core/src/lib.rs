use candle_metal_kernels::*;
use metal::{Buffer, Device, MTLResourceOptions};

fn read_to_vec<T: Clone>(buffer: &Buffer, n: usize) -> Vec<T> {
    let ptr = buffer.contents() as *const T;
    assert!(!ptr.is_null());
    let slice = unsafe { std::slice::from_raw_parts(ptr, n) };
    slice.to_vec()
}

fn approx(v: Vec<f32>, digits: i32) -> Vec<f32> {
    let b = 10f32.powi(digits);
    v.iter().map(|t| f32::round(t * b) / b).collect()
}

fn run_gemm<T: Clone>(
    device: &Device,
    (b, m, n, k): (usize, usize, usize, usize),
    lhs: &[T],
    lhs_stride: Vec<usize>,
    lhs_offset: usize,
    rhs: &[T],
    rhs_stride: Vec<usize>,
    rhs_offset: usize,
) -> Vec<T> {
    let kernels = Kernels::new();
    let command_queue = device.new_command_queue();
    let command_buffer = command_queue.new_command_buffer();
    let options = MTLResourceOptions::StorageModeManaged;

    let lhs = device.new_buffer_with_data(
        lhs.as_ptr() as *const core::ffi::c_void,
        std::mem::size_of_val(lhs) as u64,
        options,
    );
    let rhs = device.new_buffer_with_data(
        rhs.as_ptr() as *const core::ffi::c_void,
        std::mem::size_of_val(rhs) as u64,
        options,
    );
    let length = b * m * n;
    let output = device.new_buffer((length * core::mem::size_of::<T>()) as u64, options);
    call_gemm(
        device,
        command_buffer,
        &kernels,
        "sgemm",
        (b, m, n, k),
        &lhs_stride,
        lhs_offset,
        &lhs,
        &rhs_stride,
        rhs_offset,
        &rhs,
        &output,
    )
    .unwrap();
    command_buffer.commit();
    command_buffer.wait_until_completed();

    read_to_vec(&output, length)
}

fn test_gemm(device: &Device) {
    let (b, m, n, k) = (1, 2, 4, 3);
    let lhs_stride = vec![m * k, k, 1];
    let lhs: Vec<f32> = (0..b * m * k).map(|f| f as f32).collect();
    let rhs_stride = vec![n * k, n, 1];
    let rhs: Vec<f32> = (0..b * n * k).map(|f| f as f32).collect();
    let results = run_gemm(
        device,
        (b, m, n, k),
        &lhs,
        lhs_stride,
        0,
        &rhs,
        rhs_stride,
        0,
    );
    assert_eq!(
        approx(results, 4),
        vec![20.0, 23.0, 26.0, 29.0, 56.0, 68.0, 80.0, 92.0]
    );

    let (b, m, n, k) = (2, 2, 4, 3);
    let lhs_stride = vec![m * k, k, 1];
    let lhs: Vec<f32> = (0..b * m * k).map(|f| f as f32).collect();
    let rhs_stride = vec![n * k, n, 1];
    let rhs: Vec<f32> = (0..b * n * k).map(|f| f as f32).collect();
    let results = run_gemm(
        device,
        (b, m, n, k),
        &lhs,
        lhs_stride,
        0,
        &rhs,
        rhs_stride,
        0,
    );
    assert_eq!(
        approx(results, 4),
        vec![
            20.0, 23.0, 26.0, 29.0, 56.0, 68.0, 80.0, 92.0, 344.0, 365.0, 386.0, 407.0, 488.0,
            518.0, 548.0, 578.0
        ]
    );

    // OFFSET
    let (b, m, n, k) = (2, 2, 4, 3);
    let lhs_stride = vec![m * k, k, 1];
    let lhs: Vec<f32> = (0..b * m * k).map(|f| f as f32).collect();
    let rhs_stride = vec![n * k, n, 1];
    let rhs: Vec<f32> = (0..b * n * k).map(|f| f as f32).collect();
    // Manually set batch_size=1 and offset 12 elements * 4 the number of bytes for f32
    let results = run_gemm(
        device,
        (1, m, n, k),
        &lhs,
        lhs_stride,
        0,
        &rhs,
        rhs_stride,
        12 * 4,
    );
    assert_eq!(
        approx(results, 4),
        vec![56.0, 59.0, 62.0, 65.0, 200.0, 212.0, 224.0, 236.0]
    );
}

pub fn run_metal_tests() {
    for dev in Device::all() {
        println!("found {:?}", &dev);
    }

    let device = Device::system_default().unwrap();

    println!("");
    println!("using {:?}", &device);

    test_gemm(&device);
}
