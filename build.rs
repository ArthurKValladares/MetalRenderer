use std::process::Command;

fn main() {
    // TODO: This will be added to shadertools later? Maybe with some SPIR-V middleman options as well
    Command::new("xcrun")
        .args(["-sdk", "macosx"])
        .arg("metal")
        .args(["-c", "shaders/triangle.metal"])
        .args(["-o", "shaders/air/triangle.air"])
        .output()
        .expect("failed to execute process");
    Command::new("xcrun")
        .args(["-sdk", "macosx"])
        .arg("metallib")
        .arg("shaders/air/triangle.air")
        .args(["-o", "shaders/lib/triangle.metallib"])
        .output()
        .expect("failed to execute process");
}
