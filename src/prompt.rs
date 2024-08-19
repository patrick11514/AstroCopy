pub mod prompt {
    use std::{
        env,
        fmt::Write,
        fs::{self, DirEntry},
        io,
        path::{Path, PathBuf},
        time::Duration,
        vec,
    };

    use anyhow::Error;
    use clap::Parser;
    use console::style;
    use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select};
    use indicatif::{ProgressBar, ProgressState, ProgressStyle};

    use crate::args::args::Args;

    fn get_path() -> io::Result<String> {
        let args = Args::parse();
        let mut path = args.path;

        if path == "<NONE_PATH>" {
            path = env::current_dir()?.into_os_string().into_string().unwrap();
        }

        Ok(path)
    }

    fn prompt_path(theme: &ColorfulTheme) -> anyhow::Result<String> {
        let input = get_string(theme, "Enter path to photos", get_path()?.as_str());
        let path = Path::new(input.as_str());

        if !path.exists() {
            return Err(Error::msg("Path doesn't exits"));
        }

        Ok(input)
    }

    fn get_string(theme: &ColorfulTheme, text: &str, initial: &str) -> String {
        Input::<String>::with_theme(theme)
            .with_prompt(text)
            .with_initial_text(initial)
            .interact_text()
            .unwrap()
    }

    struct FolderData {
        entry: DirEntry,
        created: Duration,
    }

    pub fn select_folders(theme: &ColorfulTheme, path: &String) -> io::Result<Vec<DirEntry>> {
        let path = Path::new(path);

        let mut folders: Vec<FolderData> = Vec::new();

        for dir in path.read_dir()? {
            match dir {
                Ok(dir) => {
                    if dir.path().is_file() {
                        continue;
                    };

                    let metadata = dir.metadata().unwrap();
                    let created = metadata.modified().unwrap();

                    folders.push(FolderData {
                        entry: dir,
                        created: created.elapsed().unwrap(),
                    })
                }
                Err(_) => {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        "Unable to read directory!",
                    ))
                }
            }
        }

        folders.sort_by(|a, b| a.created.partial_cmp(&b.created).unwrap());

        let result = MultiSelect::with_theme(theme)
            .with_prompt("Select folders with data")
            .items(
                &folders
                    .iter()
                    .map(|folder| folder.entry.file_name().into_string().unwrap())
                    .collect::<Vec<String>>(),
            )
            .interact()
            .unwrap();

        let mut selected_folders: Vec<DirEntry> = Vec::new();

        for (index, folder) in folders.into_iter().enumerate() {
            if result.contains(&index) {
                selected_folders.push(folder.entry);
            }
        }

        Ok(selected_folders)
    }

    #[derive(PartialEq, Eq)]
    enum FileType {
        Light,
        Dark,
        Flat,
        Bias,
        Unknown,
    }

    struct FileFolder {
        folder_type: FileType,
        entry: PathBuf,
    }

    impl FileFolder {
        pub fn get_format(&self) -> String {
            format!(
                "{} - {}",
                style(self.entry.to_str().unwrap()).yellow().bold(),
                match self.folder_type {
                    FileType::Light => style("Light").white().bold(),
                    FileType::Dark => style("Dark").black().bold(),
                    FileType::Flat => style("Flat").yellow().bold(),
                    FileType::Bias => style("Bias").red().bold(),
                    FileType::Unknown => style("Unknown").blue().bold(),
                }
            )
        }
    }

    fn scan_folders(folders: Vec<DirEntry>) -> io::Result<Vec<FileFolder>> {
        let mut detected_folders: Vec<FileFolder> = Vec::new();

        for folder in folders.into_iter() {
            let path = folder.path();

            let mut found = false;

            for item in path.read_dir().unwrap() {
                match item {
                    Ok(entry) => {
                        let name = entry.file_name();
                        let name = name.to_str().unwrap();

                        if name.starts_with("Light") {
                            detected_folders.push(FileFolder {
                                folder_type: FileType::Light,
                                entry: folder.path(),
                            });
                            found = true;
                            break;
                        } else if name.starts_with("Flat") {
                            detected_folders.push(FileFolder {
                                folder_type: FileType::Flat,
                                entry: folder.path(),
                            });
                            found = true;
                            break;
                        } else if name.starts_with("Dark") {
                            detected_folders.push(FileFolder {
                                folder_type: FileType::Dark,
                                entry: folder.path(),
                            });
                            found = true;
                            break;
                        } else if name.starts_with("Bias") {
                            detected_folders.push(FileFolder {
                                folder_type: FileType::Bias,
                                entry: folder.path(),
                            });
                            found = true;
                            break;
                        }
                    }
                    Err(_) => {
                        return Err(io::Error::new(
                            io::ErrorKind::Other,
                            "Unable to read directory!",
                        ))
                    }
                }
            }
            if !found {
                detected_folders.push(FileFolder {
                    folder_type: FileType::Unknown,
                    entry: folder.path(),
                })
            }
        }

        Ok(detected_folders)
    }

    fn correct_folders(theme: &ColorfulTheme, folders: &mut Vec<FileFolder>) -> anyhow::Result<()> {
        let folder_type_options = vec![
            style("Light").white().bold(),
            style("Dark").black().bold(),
            style("Flat").yellow().bold(),
            style("Bias").red().bold(),
        ];

        'mainLoop: loop {
            println!(
                "{}",
                style("Here is a list of folders, and their detected type:")
                    .cyan()
                    .bold()
            );

            for folder in folders.iter() {
                println!("{}", folder.get_format());
            }

            let confirmed = Confirm::with_theme(theme)
                .with_prompt("Is folder types correct? (Unknown folders will be ignored)")
                .interact()
                .unwrap();

            if confirmed {
                break;
            }

            loop {
                let selected = Select::with_theme(theme)
                    .with_prompt("Select folder to edit")
                    .items(
                        &folders
                            .iter()
                            .map(|folder| folder.get_format())
                            .collect::<Vec<String>>(),
                    )
                    .item(style("Edit done - EXIT").red().bold())
                    .interact()
                    .unwrap();

                if selected == folders.len() {
                    break 'mainLoop;
                }

                let folder_type = Select::with_theme(theme)
                    .with_prompt("Select folder to edit")
                    .items(&folder_type_options)
                    .default(match folders[selected].folder_type {
                        FileType::Light => 0,
                        FileType::Dark => 1,
                        FileType::Flat => 2,
                        FileType::Bias => 3,
                        FileType::Unknown => 4,
                    })
                    .interact()
                    .unwrap();

                folders[selected].folder_type = match folder_type {
                    0 => FileType::Light,
                    1 => FileType::Dark,
                    2 => FileType::Flat,
                    3 => FileType::Bias,
                    _ => FileType::Unknown,
                };
            }
        }

        Ok(())
    }

    fn copy_folders(
        theme: &ColorfulTheme,
        path: String,
        folders: &Vec<FileFolder>,
        object_name: String,
        captured: String,
    ) -> anyhow::Result<()> {
        let result = Confirm::with_theme(theme)
            .with_prompt("Do you want to proceed with copying?")
            .interact()
            .unwrap();

        if !result {
            return Err(anyhow::Error::msg("Canceled by user."));
        }

        let path = Path::new(path.as_str());
        let path = path.join(format!("{}-{}", object_name, captured));

        match fs::create_dir(path.clone()) {
            Err(err) => match err.kind() {
                io::ErrorKind::AlreadyExists => {
                    let result = Confirm::with_theme(theme)
                        .with_prompt(format!(
                            "{}",
                            style(format!(
                                "Folder {} already exists, do you want to continue?",
                                path.to_str().unwrap()
                            ))
                            .red()
                            .bold(),
                        ))
                        .interact()
                        .unwrap();

                    if !result {
                        return Err(anyhow::Error::msg("Canceled by user."));
                    }
                }
                _ => return Err(err.into()),
            },
            _ => (),
        }

        for folder in folders.iter() {
            let folder_name = match folder.folder_type {
                FileType::Unknown => continue,
                FileType::Light => "Light",
                FileType::Dark => "Dark",
                FileType::Flat => "Flat",
                FileType::Bias => "Bias",
            };

            let folder_path = path.join(folder_name);

            if !folder_path.exists() {
                match fs::create_dir(folder_path.clone()) {
                    Err(err) => return Err(err.into()),
                    _ => (),
                }
            }

            //copy files
            let mut file_content_to_copy: Vec<DirEntry> = Vec::new();
            for file in folder.entry.read_dir().unwrap() {
                match file {
                    Ok(entry) => file_content_to_copy.push(entry),
                    Err(err) => return Err(err.into()),
                }
            }

            let progress = ProgressBar::new(file_content_to_copy.len() as u64);
            progress.set_style(ProgressStyle::with_template("{msg} {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>7}/{len:7} ({eta})")
            .unwrap()
            .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
            .progress_chars("#>-"));

            let msg = folder
                .entry
                .file_name()
                .unwrap()
                .to_owned()
                .into_string()
                .unwrap();

            progress.set_message(msg.clone());

            for (index, entry) in file_content_to_copy.into_iter().enumerate() {
                let from = entry.path();
                let to = folder_path.join(entry.file_name());
                match fs::copy(from, to) {
                    Err(err) => return Err(err.into()),
                    _ => (),
                }

                progress.set_position(index as u64 + 1);
            }

            progress.finish_with_message(format!("✅ {}", msg))
        }

        let result = Confirm::with_theme(theme)
            .with_prompt(format!(
                "{}",
                style("Do you want to delete old folders?",).red().bold(),
            ))
            .interact()
            .unwrap();

        if result {
            let progress = ProgressBar::new(folders.len() as u64);
            progress.set_style(ProgressStyle::with_template("{msg} {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>7}/{len:7} ({eta})")
            .unwrap()
            .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
            .progress_chars("#>-"));

            progress.set_message("Deleting folders...");

            for (index, folder) in folders.into_iter().enumerate() {
                if folder.folder_type == FileType::Unknown {
                    continue;
                }

                match fs::remove_dir_all(folder.entry.as_path()) {
                    Err(err) => return Err(err.into()),
                    _ => progress.set_position(index as u64 + 1),
                }
            }

            progress.finish_with_message("Done");
        }

        Ok(())
    }

    pub fn show_prompt() -> anyhow::Result<()> {
        //default theme
        let theme = &ColorfulTheme::default();

        //get current path/path from arg
        let path = prompt_path(theme)?;
        //get object name: M51, Andromeda etc...
        let object_name = get_string(theme, "Enter name of captured object", "");
        //get date
        let captured = get_string(
            theme,
            "Enter capture time (ideally in format YYYY-MM-DD)",
            "",
        );

        //get folders in path and scan them for their type
        let mut folders = scan_folders(select_folders(theme, &path)?)?;
        //print folders + ask for correcting them, if needed
        correct_folders(theme, &mut folders)?;
        //create folder and copy files from that folder
        copy_folders(theme, path, &folders, object_name, captured)?;

        println!("{}", style("Copying done!").green().bold());

        Ok(())
    }
}
