use std::{
    cmp::Ordering,
    fs::File,
    path::{Path, PathBuf},
    process::Command,
    time::SystemTime,
};

use chrono::{offset::Utc, DateTime};
use flate2::write::GzEncoder;

fn main() {
    let datetime: DateTime<Utc> = SystemTime::now().into();
    let datetime_str = datetime.format("%Y-%m-%d-%H-%M-%S-%f").to_string();
    let tempdir = tmp_path().join(datetime_str);
    let source_dir = tempdir.join("source");
    let rust_tar = tempdir.join("rust_no_compression_tar.tar");
    let system_tar = tempdir.join("system_no_compression_tar.tar");

    let fixture = fixture_path().join("ruby-compiled-3.1.6.tgz");

    println!("- Creating source dir {}", source_dir.display());
    std::fs::create_dir_all(&source_dir).expect("create source dir");
    println!("- Untarring fixture {} to source dir", fixture.display());
    untar_to_dir(&fixture, &source_dir);

    println!("- Creating rust_tar {}", rust_tar.display());
    ruby_tar_no_gzip(&source_dir, &rust_tar);

    println!("- Creating system_tar {}", system_tar.display());
    system_tar_no_gzip(&source_dir, &system_tar);

    let system_tar_metdata = File::open(&system_tar)
        .expect("open system tar file")
        .metadata()
        .expect("get metadata");
    let rust_tar_metdata = File::open(&rust_tar)
        .expect("open system tar file")
        .metadata()
        .expect("get metadata");

    println!(
        "system_tar no compression size: {}",
        system_tar_metdata.len()
    );
    println!("rust_tar no compression size: {}", rust_tar_metdata.len());

    match system_tar_metdata.len().cmp(&rust_tar_metdata.len()) {
        Ordering::Equal => {
            println!("## system_tar and rust_tar are the SAME size (no compression)")
        }
        Ordering::Less => {
            println!("## system_tar is smaller than rust_tar (no compression)\n :(!!!")
        }
        Ordering::Greater => {
            println!("## rust_tar is smaller than system_tar (no compression)\n :)!!!")
        }
    }

    println!(
        "# Comparing gzip compression using RUST tar {}",
        rust_tar.display()
    );

    let rust_tar_gz = tempdir.join("rust_gzip_a_tar.tar.gz");
    let system_tar_gz = tempdir.join("system_gzip_a_tar.tar.gz");
    println!(
        "- RUST gzip {} to {}",
        rust_tar.display(),
        rust_tar_gz.display()
    );
    rust_gzip(&rust_tar, &rust_tar_gz);
    println!("- Done");

    println!(
        "- SYSTEM gzip {} to {}",
        system_tar.display(),
        system_tar_gz.display()
    );
    system_gzip(&rust_tar, &system_tar_gz);
    println!("- Done");

    let system_tar_gz_metdata = File::open(&system_tar_gz)
        .expect("open system tar file")
        .metadata()
        .expect("get metadata");
    let rust_tar_gz_metdata = File::open(&rust_tar_gz)
        .expect("open system tar file")
        .metadata()
        .expect("get metadata");

    println!("- system_tar_gz size: {}", system_tar_gz_metdata.len());
    println!("- rust_tar_gz size: {}", rust_tar_gz_metdata.len());

    match system_tar_gz_metdata.len().cmp(&rust_tar_gz_metdata.len()) {
        Ordering::Less => println!("## system_tar_gz is smaller than rust_tar_gz (gzip)\n :("),
        Ordering::Equal => println!("## system_tar_gz and rust_tar_gz are the SAME size (gzip)"),
        Ordering::Greater => {
            println!("## rust_tar_gz is smaller than system_tar_gz (gzip)\n :)")
        }
    };

    println!("## Comparing tar+gz at the same time using RUST and system");

    let rust_tar_gz = tempdir.join("rust_tar_gzip_one_operation.tar.gz");
    println!("- RUST tar+gzip {}", rust_tar_gz.display());
    rust_tar_and_gzip(&source_dir, &rust_tar_gz);
    println!("- Done");

    let system_tar_gz = tempdir.join("system_tar_gzip_one_operation.tar.gz");
    println!("- System tar+gzip");
    system_tar_and_gzip(&source_dir, &system_tar_gz);
    println!("- Done");
}

fn rust_tar_and_gzip(dir_to_tar: &Path, destination_tar_gz: &Path) {
    let mut enc = GzEncoder::new(
        File::create(destination_tar_gz).expect("create tar.gz file"),
        flate2::Compression::best(),
    );

    tar::Builder::new(&mut enc)
        .append_dir_all("", dir_to_tar)
        .expect("add dir to tar");
}

fn system_tar_and_gzip(dir_to_tar: &Path, destination_tar_gz: &Path) {
    let mut cmd = Command::new("bash");
    cmd.arg("-c");
    cmd.arg(
        [
            format!("cd {}", dir_to_tar.display()),
            format!("tar -czf {} *", destination_tar_gz.display()),
        ]
        .join(" && "),
    );

    println!("  - Running tar+gzip command: {:?}", cmd);
    let output = cmd.output().expect("run tar+gzip command");

    println!("  - stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("  - stderr: {}", String::from_utf8_lossy(&output.stderr));
    if output.status.success() {
        println!("  - tar+gzip command succeeded");
    } else {
        panic!("  - tar+gzip command failed");
    }
}

fn system_gzip(tar_file: &Path, destination_tar_gz: &Path) {
    let mut cmd = Command::new("bash");
    cmd.arg("-c");
    cmd.arg(format!(
        "gzip --best --stdout {from} > {to}",
        from = tar_file.display(),
        to = destination_tar_gz.display()
    ));

    println!("  - Running gzip command: {:?}", cmd);
    let output = cmd.output().expect("run gzip command");

    println!("  - stderr: {}", String::from_utf8_lossy(&output.stderr));
    if output.status.success() {
        println!("  - gzip command succeeded");
    } else {
        panic!("  - gzip command failed");
    }
}

fn rust_gzip(tar_file: &Path, destination_tar_gz: &Path) {
    let mut tar_file = File::open(tar_file).expect("open tar file");

    let mut enc = flate2::write::GzEncoder::new(
        File::create(destination_tar_gz).expect("create tar.gz file"),
        flate2::Compression::best(),
    );

    std::io::copy(&mut tar_file, &mut enc).expect("copy tar to gzip");
}

fn ruby_tar_no_gzip(dir_to_tar: &Path, destination_tar: &Path) {
    tar::Builder::new(File::create(destination_tar).expect("create tar file"))
        .append_dir_all("", dir_to_tar)
        .expect("add dir to tar");
}

fn system_tar_no_gzip(dir_to_tar: &Path, destination_tar: &Path) {
    let mut cmd = Command::new("bash");
    cmd.arg("-c");
    cmd.arg(
        [
            format!("cd {}", dir_to_tar.display()),
            format!("tar -cf {} *", destination_tar.display()),
        ]
        .join(" && "),
    );

    println!("  - Running tar command: {:?}", cmd);
    let output = cmd.output().expect("run tar command");

    println!("  - stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("  - stderr: {}", String::from_utf8_lossy(&output.stderr));
    if output.status.success() {
        println!("  - tar command succeeded");
    } else {
        panic!("  - tar command failed");
    }
}

fn untar_to_dir(from: &Path, to: &Path) {
    let file = std::fs::File::open(from).expect("open file");
    let tar = flate2::read::GzDecoder::new(file);
    let mut archive = tar::Archive::new(tar);
    archive.unpack(to).expect("unpack tar.gz");
}

fn fixture_path() -> PathBuf {
    workspace_path().join("fixtures")
}

fn tmp_path() -> PathBuf {
    workspace_path().join("tmp")
}

fn workspace_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}
