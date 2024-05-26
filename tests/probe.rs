use iou::Probe;
use uring_sys::io_uring_op;

#[test]
fn probe() {
    let probe = Probe::new().unwrap();
    assert!(probe.supports(io_uring_op::IORING_OP_NOP));
}
