use std::env;
use std::fs;
use std::path::Path;


type Result<A> = std::result::Result<A, Box<dyn std::error::Error>>;
type ReplaceList = Vec<(&'static str, String)>;

fn get_replacements(api_name: &str) -> ReplaceList {
    let api_path = format!("crate::apis::{}::apis", api_name);
    let models_path = format!("crate::apis::{}::models", api_name);
    return vec![
        ("crate::apis", api_path),
        ("crate::models", models_path),
        ("-PARAM-backup_id: &str", "backup_id: crate::types::BackupId".to_string()),
        ("-PARAM-cluster_id: &str", "cluster_id: crate::types::ClusterId".to_string()),
        ("-PARAM-organization_id: &str", "organization_id: crate::types::OrgId".to_string()),
        ("-PARAM-project_id: &str", "project_id: crate::types::ProjectId".to_string()),            
        ("-PARAM-", "".to_string()),    
    ];
}

fn copy_file(replacements: &ReplaceList, src: &Path, dst: &Path) -> Result<()> {    
    println!("{:?} -> {:?}\n", src, dst);

    let mut contents = fs::read_to_string(src)?;
    for r in replacements.iter() {
        contents = contents.replace(r.0, r.1.as_str());        
    } 

    fs::write(dst, contents)?;
    Ok(())
}

fn iterate_directory(replacements: &ReplaceList, src: &Path, dst: &Path) -> Result<()> { 
    println!("D {:?} -> {:?}\n", src, dst);
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let path = entry?.path();
        if let Some(os_str) = path.file_name() {
            if let Some(name) = os_str.to_str() {
                let dst_path = &dst.join(name);
                if path.is_dir() {
                    iterate_directory(&replacements, &path, &dst_path)?
                } else if path.is_file() {
                    copy_file(&replacements, &path, &dst_path)?
                }
            }
        }
    }
    Ok(())
}

fn iterate_starting_directory(replacements: &ReplaceList, src: &Path, dst: &Path) -> Result<()> { 
    fs::create_dir_all(dst)?;
    copy_file(replacements, &src.join("lib.rs"), &dst.join("mod.rs"))?;
    iterate_directory(replacements, &src.join("apis"), &dst.join("apis"))?;
    iterate_directory(replacements, &src.join("models"), &dst.join("models"))?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        panic!("expected 3 args: api_name src_dir dst_dir");    
    }
    let replacements = get_replacements(&args[1]);
    match iterate_starting_directory(&replacements, Path::new(&args[2]), Path::new(&args[3]))   {
        Ok(_) => {
            println!("OK!");
        }, 
        Err(err) => {
            println!("Err! {}", err);
        }
    }
}
