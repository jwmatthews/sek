use crate::config;
use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{
    fmt, fs, io,
    path::{Path, PathBuf},
};

use kube::config::Kubeconfig;
use kube::config::NamedContext;


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterEntry {
    pub kubeconfig_path: String,
    pub admin_password: String,
    pub cluster_api_endpoint: String,
}
impl Default for ClusterEntry {
    fn default() -> ClusterEntry {
        ClusterEntry {
            kubeconfig_path: String::from("kube"),
            admin_password: String::from("admin"),
            cluster_api_endpoint: String::from("api_endpoint"),
        }
    }
}
// To use the `{}` marker, the trait `fmt::Display` must be implemented
impl fmt::Display for ClusterEntry {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(
            f,
            "\n\t\tkubeconfig_path: {}, \n\t\tadmin_password: {}, \n\t\tcluster_api_endpoint: {}",
            self.kubeconfig_path.bright_purple(),
            self.admin_password.bright_purple(),
            self.cluster_api_endpoint.bright_purple()
        )
    }
}

pub fn refresh(cfg: config::Config) {
    // Learn what Agnosticd directories exist
    let potential_dirs = find_provisioned_agnosticd_dirs(cfg);
    println!("found_dirs: {:?}", potential_dirs);
    let cluster_info = parse_cluster_info(potential_dirs);
    for entry in cluster_info {
        println!(
            "\t{} : {}",
            entry.0.display().to_string().bright_blue(),
            entry.1
        )
    }
    // Learn which clusters are up
    // Update our cached info
}

fn find_provisioned_agnosticd_dirs(cfg: config::Config) -> Vec<PathBuf> {
    let mut found_dirs: Vec<PathBuf> = Vec::new();

    for r in cfg.agnosticd_resource_dirs {
        println!("Looking at: {}", r.cyan());
        match read_dirs_from_dir(&r) {
            Ok(v) => {
                println!("Inside find_provisioned_agnosticd_dirs: v = {:?}", v);
                found_dirs.extend(v);
            }
            Err(e) => {
                println!(
                    "Error {} reading {}",
                    e.to_string().red(),
                    r.to_string().cyan()
                );
            }
        }
    }
    return found_dirs;
}

fn parse_cluster_info(potential_dirs: Vec<PathBuf>) -> HashMap<PathBuf, ClusterEntry> {
    let mut cluster_info = HashMap::<PathBuf, ClusterEntry>::new();
    for d in potential_dirs {
        // Find the filenames ending in "kubeconfig" and "kubeadmin-password" and get the full paths
        // Ensure each dir has both has a file "kubeconfig" and "kubeadmin-password"
        match ensure_dir_has_cluster_info(&d) {
            Some(map) => {
                let kubeconfig = map.get("kubeconfig").unwrap().display().to_string();
                match parse_cluster_api_endpoint(&kubeconfig) {
                    Some(api_endpoint) => {
                        cluster_info.insert(
                            d,
                            ClusterEntry {
                                cluster_api_endpoint: api_endpoint,
                                // TODO: could clean up to remove unwrap
                                kubeconfig_path: kubeconfig.clone(),
                                admin_password: map
                                    .get("kubeadmin-password")
                                    .unwrap()
                                    .display()
                                    .to_string(),
                            },
                        );
                    }
                    None => {
                        println!(
                            "{} {} unable to process cluster_api_endpoint",
                            "Skipping".yellow(),
                            d.display().to_string().cyan()
                        )
                    }
                }
            }
            None => {}
        }
    }
    return cluster_info;
}

fn get_named_context(target_name: &str, contexts: &Vec<NamedContext>) -> Option<NamedContext> {
    for c in contexts {
        if c.name == target_name {
            return Some(c.clone());
        }
    }
    return None;
}

fn parse_cluster_api_endpoint(kubeconfig_filename: &str) -> Option<String> {
    let r = Kubeconfig::read_from(kubeconfig_filename);

    r.ok().map(|kc| {
        get_current_cluster(&kc).and_then(|cluster_name| {
            println!(
                "{} {}",
                "info".yellow(),
                &cluster_name);
            kc.clusters.iter().find(|&c| c.name == cluster_name).and_then(|c| {
                c.cluster.as_ref().and_then(|x| {
                    println!("{}: {}", "server".yellow(), x.server.as_ref().unwrap().cyan());
                    x.server.clone()
                })
            })
        })
    }).flatten()
}

fn _ugly_parse_cluster_api_endpoint(kubeconfig_filename: &str) -> Option<String> {
    let r = Kubeconfig::read_from(kubeconfig_filename);
    match r {
        Ok(kc) => {
            return match get_current_cluster(&kc) {
                Some(cluster_name) => {
                    println!(
                        "{} {}",
                        "info".yellow(),
                        &cluster_name);
                    let cluster = kc.clusters.iter().find(|&c| c.name == cluster_name);
                    //println!("clusters: {:?}", cluster);
                    match cluster {
                        Some(c) => {
                            match &c.cluster {
                                Some(x) => {
                                    // TODO Below as_ref/unwrap feels wrong
                                    println!("{}: {}", "server".yellow(), x.server.as_ref().unwrap().cyan());
                                    x.server.clone()
                                }
                                None => { None }
                            }
                        }
                        None => {
                            println!("{} {}", "error".red(), "Unable to find cluster");
                            None
                        }
                    }
                }
                None => {
                    println!("{} {}", "error".red(), "Unable to find context");
                    None
                }
            }
        }
        Err(e) => {
            println!("{}", e);
            None
        }
    }
}

fn get_current_cluster(kc: &Kubeconfig) -> Option<String> {
    let current_context = kc.current_context.as_ref().unwrap();
    println!("{} {}", "info".yellow(), &current_context);
    let ctx = get_named_context(current_context, &kc.contexts);
    return match ctx {
        Some(c) => {
            Some(c.context.unwrap().cluster)
        }
        None => {
            println!("{} {}", "error".red(), "Unable to find context");
            None
        }
    }
}

fn ensure_dir_has_cluster_info(dir: &PathBuf) -> Option<HashMap<String, PathBuf>> {
    let endings = vec!["kubeconfig", "kubeadmin-password"];
    let mut map = HashMap::<String, PathBuf>::new();
    let files =
        read_filenames_from_dir(dir).expect(format!("Error from {}", dir.display()).as_str());
    // Find the associated path for each ending, we assume there is only one entry of this type in a directory
    for ending in endings.clone() {
        for f in files.clone() {
            if f.is_file()
                && f.file_name()
                    .is_some_and(|x| x.to_str().is_some_and(|x| x.contains(ending)))
            {
                map.insert(String::from(ending), f);
            }
        }
    }
    for ending in endings.clone() {
        if !map.contains_key(ending) {
            println!(
                "{} is missing an entry for {}",
                dir.display().to_string().yellow(),
                ending.blue()
            );
            return None;
        }
    }
    return Some(map);
}

// Example from: https://stackoverflow.com/questions/37439327/how-to-write-a-function-that-returns-vecpath
pub fn read_filenames_from_dir<P>(path: P) -> Result<Vec<PathBuf>, io::Error>
where
    P: AsRef<Path>,
{
    fs::read_dir(path)?
        .into_iter()
        .map(|x| x.map(|entry| entry.path()))
        .collect()
}

pub fn read_dirs_from_dir<P>(path: P) -> Result<Vec<PathBuf>, io::Error>
where
    P: AsRef<Path>,
{
    fs::read_dir(path)?
        .into_iter()
        .map(|x| x.map(|entry| entry.path()))
        .filter(|x| x.as_ref().is_ok_and(|f| f.is_dir()))
        // Skip the archive directories
        .filter(|x| {
            x.as_ref().is_ok_and(|f| {
                f.file_name()
                    .is_some_and(|x| x.to_str().is_some_and(|x| !x.contains("archive")))
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    const TEST_DATA: &str = "test_data/agnosticd";

    fn get_test_config() -> Config {
        Config {
            sek_home: String::from(""),
            agnosticd_resource_dirs: vec![TEST_DATA.to_string()],
        }
    }

    #[test]
    fn test_find_provisioned_agnosticd_dirs() {
        // Tests we found potential directories
        let cfg = get_test_config();
        let found_dirs = find_provisioned_agnosticd_dirs(cfg);
        assert_eq!(found_dirs.len(), 9);
    }

    #[test]
    fn test_ensure_dir_has_file_endings() {
        match ensure_dir_has_cluster_info(&PathBuf::from("test_data/agnosticd/jwm0727ocp412b/")) {
            Some(_) => {
                assert!(true)
            }
            None => {
                assert!(false)
            }
        }
        match ensure_dir_has_cluster_info(&PathBuf::from("test_data/agnosticd/partialentry3/")) {
            Some(_) => {
                assert!(false)
            }
            None => {
                assert!(true)
            }
        }
    }

    #[test]
    fn test_parse_cluster_info() {
        // Tests we are filtering from the potential directories to those with complete information
        let cfg = get_test_config();
        let found_dirs = find_provisioned_agnosticd_dirs(cfg);
        let cluster_info = parse_cluster_info(found_dirs);
        assert_eq!(cluster_info.len(), 4);
        let expected = [
            "test_data/agnosticd/jwm0727ocp412b",
            "test_data/agnosticd/jwm0603ocp413a",
            "test_data/agnosticd/jwm0728ocp412b",
            "test_data/agnosticd/jwm0706ocp413a",
        ];
        for d in expected {
            assert!(cluster_info.contains_key(Path::new(d)));
        }
    }

    #[test]
    fn test_parse_cluster_api_endpoint() {
        // Positive test-case, expect success
        let kubeconfig =
            "test_data/agnosticd/jwm0706ocp413a/ocp4-cluster_jwm0603ocp413a_kubeconfig";
        let expected_server = "https://api.cluster-jwm0603ocp413a.jwm0603ocp413a.mg.somewhere.com:6443";

        match parse_cluster_api_endpoint(kubeconfig) {
            Some(endpoint) => {
                assert_eq!(endpoint, expected_server);
            }
            None => {
                assert!(false);
            }
        }
        // Negative test-case, expect it can't parse
        let kubeconfig = "test_data/agnosticd/jwm_bad_kubeconfig/ocp4-cluster_bad_kubeconfig";
        match parse_cluster_api_endpoint(kubeconfig) {
            Some(_) => {
                assert!(false);
            }
            None => {
                assert!(true);
            }
        }
    }
}
