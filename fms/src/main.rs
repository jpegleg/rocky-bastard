use std::env;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::fs::File;
use std::io;
use std::io::Read;
use std::os::unix::fs::MetadataExt;
use std::fs::OpenOptions;
use sha3::{Shake256, digest::{Update, ExtendableOutput}};
use chrono::{NaiveDateTime, DateTime, Utc};
use users::{get_user_by_uid, get_group_by_gid};

fn review(file_path: &str)  {
    let file_path = Path::new(file_path);
    let metadata = file_path.metadata().expect("Failed to read file metadata");
    let mut file = File::open(&file_path).expect("Failed to open the file");
    let mut bytes = Vec::new();

    file.read_to_end(&mut bytes).expect("Failed to read the file");

    let num_bytes = bytes.len();
    let num_bits = num_bytes * 8;
    let byte_distribution = bytes.iter().collect::<std::collections::HashSet<_>>().len() as f64 / num_bytes as f64;

    let file_is_open = match OpenOptions::new()
        .read(true)
        .write(true)
        .open(file_path)
        {
            Ok(_) => false,
            Err(_) => true,
        };

     let chronox: String = Utc::now().to_string();
     let mut hasher = Shake256::default();
     hasher.update(&bytes);
     let mut resulto = hasher.finalize_xof();
     let mut shake256 = [0u8; 10];
     let _ = resulto.read(&mut shake256);
     println!("{{");
     println!("{:?}: {{", file_path);
     println!("  \"Checksum SHA3 SHAKE256 10\": \"{:?}\",", shake256);
     println!("  \"Report time\": \"{}\",", chronox.to_string());
     let num_io_blocks = metadata.blocks();
     println!("  \"Number of IO blocks\": \"{}\",", num_io_blocks);
     let blocksize = metadata.blksize();
     println!("  \"Block size\": \"{}\",", blocksize);
     let inode = metadata.ino();
     println!("  \"Inode\": \"{}\",", &inode);
     println!("  \"Total as bytes\": \"{}\",", &num_bytes);
     println!("  \"Total as kilobytes\": \"{}\",", &num_bytes / 1024);
     println!("  \"Total as megabytes\": \"{}\",", &num_bytes / (1024 * 1024));
     println!("  \"Total as bits\": \"{}\",", num_bits);
     println!("  \"Byte distribution\": \"{}\",", byte_distribution);
     let created: DateTime<Utc> = DateTime::from(metadata.created().expect("Failed to get created timestamp."));
     let modified: DateTime<Utc> = DateTime::from(metadata.modified().expect("failed to get modified timestamp."));
     let access: DateTime<Utc> = DateTime::from(metadata.accessed().expect("failed to get accessed timestamp."));
     let changed: DateTime<Utc> = {
         let ctime = metadata.ctime();
         let ctimesec = metadata.ctime_nsec() as u32;
         let naive_datetime = NaiveDateTime::from_timestamp_opt(ctime, ctimesec).expect("Invalid changed timestamp");
         DateTime::<Utc>::from_utc(naive_datetime, Utc)
     };

     println!("  \"Created timestamp (UTC)\": \"{}\",", created);
     println!("  \"Modified timestamp (UTC)\": \"{}\",", modified);
     println!("  \"Accessed timestamp (UTC)\": \"{}\",", access);
     println!("  \"Changed timestamp (UTC)\": \"{}\",", changed);

     let permission = metadata.permissions();
     let mode = permission.mode();
     println!("  \"Permissions\": \"{:o}\",", mode);

     let uid = metadata.uid();
     let gid = metadata.gid();

     let owner = match get_user_by_uid(uid) {
         Some(user) => user.name().to_string_lossy().into_owned(),
         None => "-".to_string(),
     };

     let group = match get_group_by_gid(gid) {
         Some(group) => group.name().to_string_lossy().into_owned(),
         None => "-".to_string(),
     };

     println!("  \"Owner\": \"{} (uid: {})\",", owner, uid);
     println!("  \"Group\": \"{} (gid: {})\",", group, gid);

     if file_is_open {
         println!(" \"Open\": \"File is currently open by another program...\",");
     } else {
         println!("  \"Open\": \"File is not open by another program.\",");
     }

     println!(" }}");
     println!("}}");
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <path_to_check>", args[0]);
        return Ok(());
    }

    let file_path = std::env::args().nth(1).expect("failed to read file path").to_string();
    review(&file_path);
    Ok(())
}
