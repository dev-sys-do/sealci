use reqwest::multipart::{Form, Part};
use reqwest::{Client, Response};
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub async fn send_to_controller(pipeline_name: &str, repo_name: &str, actions_file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let client: Client = Client::new();

    // Lire le fichier dans un buffer
    let mut file: File = File::open(actions_file_path)?;
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Créer une partie de formulaire avec le contenu du fichier
    let file_part: Part = Part::bytes(buffer)
        .file_name(actions_file_path.file_name().unwrap().to_string_lossy().into_owned());

    // Créer le formulaire multipart et ajouter les parties
    let form: Form = Form::new()
        .text("name", pipeline_name.to_string())
        .text("repo_name", repo_name.to_string())
        .part("body", file_part);

    // Envoyer la requête POST
    let res: Response = client.post("http://controller-url/pipeline")
        .multipart(form)
        .send()
        .await?;

    println!("Response: {:?}", res);

    Ok(())
}
