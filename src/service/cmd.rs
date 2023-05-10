use std::env::{temp_dir, current_dir};
use std::fs::File;
use std::io::{Write};
use std::process::Command;
use rocket::log::{info_ as info, warn_};
use std::time::{Duration, Instant};
use std::{sync::{Arc, RwLock}, thread, collections::VecDeque};
use crate::{ERROR_1, ERROR_2, SUCCESS, PROGRAM_HASH, InscribeContent, VerifyData, InscribeIdContent, OrdOp, FileLock, FILE_LOCK_NAME, FILE_LOCK2_NAME};


lazy_static::lazy_static! {
    pub static ref CUR_ORD_INDEX: Arc<RwLock<VecDeque<usize>>> = Arc::new(RwLock::new(VecDeque::new()));
    pub static ref CUR_ORD2_INDEX: Arc<RwLock<VecDeque<usize>>> = Arc::new(RwLock::new(VecDeque::new()));
}

pub fn get_inscribe_by_number(number: i64) -> (Option<InscribeContent>, i32) {
    // let _ = ord_index(OrdOp::NUMBER);
    let lock = FileLock::lock(FILE_LOCK_NAME);
    let output = Command::new("ord")
                .arg("find-number")
                .arg(number.to_string()).output().unwrap();

    // let _ = ord_index_sub();
    drop(lock);

    if output.status.success() {
        let resp = serde_json::from_slice(&output.stdout);
        
        if resp.is_ok() {
            (Some(resp.unwrap()), SUCCESS)
        }else {
            (None, ERROR_1)
        }
        
    }else {
        warn_!("get_inscribe_by_number failed number: {}, output: {:?}", number, String::from_utf8(output.stderr));
        (None, ERROR_2)
    }
}

pub fn get_inscribe_by_id_cmd(id: &str) -> (Option<InscribeIdContent>, i32) {
    let start_time = Instant::now();
    // let _ = ord2_index(OrdOp::ID);
    let lock = FileLock::lock(FILE_LOCK2_NAME);
    info!("id: {:?}, wait index time: {:?}", id, start_time.elapsed());
    let output = Command::new("ord")
                .arg("--data-dir=/home/ubuntu/.local/share2/ord")
                .arg("find-by-id")
                .arg(id).output().unwrap();

    // let _ = ord2_index_sub();
    info!("id: {:?}, cmd end time: {:?}", id, start_time.elapsed());
    drop(lock);
    if output.status.success() {
        let resp = serde_json::from_slice(&output.stdout);
        
        if resp.is_ok() {
            (Some(resp.unwrap()), SUCCESS)
        }else {
            (None, ERROR_1)
        }
        
    }else {
        warn_!("get_inscribe_by_id failed id: {}, output: {:?}", id, String::from_utf8(output.stderr));
        (None, ERROR_2)
    }
}

// pub fn ord_index(op: OrdOp) -> usize {
//     let length = (*CUR_ORD_INDEX).read().unwrap().len();
//     if length == 0 {
//         let _ =&(*CUR_ORD_INDEX).write().unwrap().push_back(0);
//         0
//     }else {
//         match op {
//             OrdOp::ID => {thread::sleep(Duration::from_millis(10));},
//             OrdOp::NUMBER => {thread::sleep(Duration::from_millis(100));}
//         }
//         ord_index(op)
//     }
// }

// pub fn ord_index_sub() {
//     let _ = (*CUR_ORD_INDEX).write().unwrap().pop_front();
// }

// pub fn ord2_index(op: OrdOp) -> usize {
//     let length = (*CUR_ORD2_INDEX).read().unwrap().len();
//     if length == 0 {
//         let _ =&(*CUR_ORD2_INDEX).write().unwrap().push_back(0);
//         0
//     }else {
//         match op {
//             OrdOp::ID => {thread::sleep(Duration::from_millis(10));},
//             OrdOp::NUMBER => {thread::sleep(Duration::from_millis(100));}
//         }
//         ord2_index(op)
//     }
// }

// pub fn ord2_index_sub() {
//     let _ = (*CUR_ORD2_INDEX).write().unwrap().pop_front();
// }

pub fn ord_index_service() {
    // let _ = ord2_index(OrdOp::ID);
    let lock = FileLock::lock(FILE_LOCK2_NAME);
    let _ = Command::new("ord")
        .arg("--data-dir=/home/ubuntu/.local/share2/ord")
        .arg("index")
        .output().unwrap();
    // ord2_index_sub();
    drop(lock);
}

pub fn generate_proof(verify_data: &VerifyData, name: &str) -> Option<Vec<u8>> {
    let name = &name[0..name.len() - 4];
    let path = temp_dir().join(format!("{}_input.json", name));
    let mut file = File::create(&path).expect("failed create or open file");
    let data = serde_json::to_vec(verify_data).unwrap();
    let _ = file.write_all(&data);


    let compile_file = temp_dir().join(format!("{}_compiled.json", name));
    println!("compile_file: {:?}", compile_file);
    let cairo_path = current_dir().unwrap().join("verify_signature.cairo");
    println!("cur_dir: {:?}", cairo_path);

    let code = if compile_file.exists() && compile_file.metadata().unwrap().len() > 0{
        info!("compile file [{:?}] exists", &compile_file);
        SUCCESS
    }else {
        let output = Command::new("cairo-compile")
                .arg(cairo_path)
                .arg("--output")
                .arg(&compile_file)
                .output().unwrap();
        info!("compile result: {:?}", output);
        output.status.code().unwrap()
    };

    info!("after compile");

    if code == SUCCESS {
        let memory_path = temp_dir().join(format!("{}_memory.bin", name));
        let trace_path = temp_dir().join(format!("{}_trace.bin", name));
        info!("before run compile_file: {:?}", &compile_file);
        let code = if memory_path.exists() && memory_path.metadata().unwrap().len() > 0 && trace_path.exists() && trace_path.metadata().unwrap().len() > 0{
            info!("memory_path: [{:?}, len: {}] & trace_path: [{:?}] exists", &memory_path, &memory_path.metadata().unwrap().len(), &trace_path);
            SUCCESS
        }else {
            let run_result = Command::new("cairo-run")
                .arg(format!("--program={}", &compile_file.to_str().unwrap()))
                .arg("--layout=all_solidity")
                .arg(format!("--memory_file={}", &memory_path.to_str().unwrap()))
                .arg(format!("--trace_file={}", &trace_path.to_str().unwrap()))
                .arg(format!("--program_input={}", &path.to_str().unwrap()))
                .output().unwrap();
            info!("run_result: {:?}", run_result);
            run_result.status.code().unwrap()
        };
        
        info!("after run");
        if code == SUCCESS {
            let output_bin = temp_dir().join(format!("file/{}.bin", name));
            let code = if output_bin.exists() && output_bin.metadata().unwrap().len() > 0{
                info!("output_bin file [{:?}] exists", &output_bin);
                SUCCESS
            }else {
                let prove_result = Command::new("giza")
                    .arg("prove")
                    .arg(format!("--trace={}", &trace_path.to_str().unwrap()))
                    .arg(format!("--memory={}", &memory_path.to_str().unwrap()))
                    .arg(format!("--program={}", &compile_file.to_str().unwrap()))  
                    .arg(format!("--output={}", &output_bin.to_str().unwrap()))
                    .arg("--num-outputs=2")
                    .arg(format!("--program-hash={}", PROGRAM_HASH))
                    .output().unwrap();

                info!("prove_result: {:?}", prove_result);
                prove_result.status.code().unwrap()
            };
            
            if code == SUCCESS {
                // let mut output_file = File::open(&output_bin).unwrap();
                // let mut output_data = Vec::new();
                // let _ = output_file.read_to_end(&mut output_data);
                // Some(output_data)
                Some(vec![])
            }else {
                None
            }
        }else {
            None
        }
        
    }else {
        None
    }
}