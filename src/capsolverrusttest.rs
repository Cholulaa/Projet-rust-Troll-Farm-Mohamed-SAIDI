use thirtyfour::prelude::*;
use tokio;
use std::time::Duration;
use std::error::Error;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    driver.goto("https://x.com/i/flow/signup").await?;

    // Clique du bouton "créer un compte"
    let button = driver // crée un élément button 
        .query(By::XPath("/html/body/div/div/div/div[1]/div[2]/div/div/div/div/div/div[2]/div[2]/div/div/div[2]/div[2]/div/div/div/button[2]/div/span/span")) // trouve le bouton avec le XPATH
        .wait(Duration::from_secs(10), Duration::from_secs(1)) // temps d'attente pour que le bouton charge
        .single()
        .await?;
    button.click().await?; // clique du bouton
    println!("Clicked 'créer un compte' button");

    // Remplissage du champ "Nom et prénom" 
    let elem_nom_prenom = driver.find(By::XPath("/html/body/div/div/div/div[1]/div[2]/div/div/div/div/div/div[2]/div[2]/div/div/div[2]/div[2]/div[1]/div/div[2]/div[1]/label/div/div[2]/div/input")).await?; // trouve le champ "nom et prénom" avec le xpath
    elem_nom_prenom.send_keys("Test gros bg").await?;
    println!("Filled 'Nom et prénom'");

    // Remplissage du champ "Email"
    let elem_mail = driver.find(By::XPath("/html/body/div/div/div/div[1]/div[2]/div/div/div/div/div/div[2]/div[2]/div/div/div[2]/div[2]/div[1]/div/div[2]/div[2]/label/div/div[2]/div/input")).await?; // trouve le champ "nom et prénom" avec le xpath
    elem_mail.send_keys("grosb@zouz.com").await?;
    println!("Filled 'Email'");

    // Sélection du mois de naissance
    let month_dropdown = driver
        .query(By::XPath("//select[@id='SELECTOR_1']"))
        .wait(Duration::from_secs(10), Duration::from_secs(1))
        .single()
        .await?;
    month_dropdown.click().await?;
    println!("Clicked month dropdown");

    let march_option = driver
        .find(By::XPath("//select[@id='SELECTOR_1']/option[@value='3']"))
        .await?;
    march_option.click().await?;
    println!("Selected 'March'");

    // Sélection du jour de naissance
    let day_dropdown = driver
        .query(By::XPath("//select[@id='SELECTOR_2']"))
        .wait(Duration::from_secs(10), Duration::from_secs(1))
        .single()
        .await?;
    day_dropdown.click().await?;
    println!("Clicked day dropdown");

    let day_02_option = driver
        .find(By::XPath("//select[@id='SELECTOR_2']/option[@value='2']"))
        .await?;
    day_02_option.click().await?;
    println!("Selected '02'");

    // Sélection de l'année de naissance
    let year_dropdown = driver
        .query(By::XPath("//select[@id='SELECTOR_3']"))
        .wait(Duration::from_secs(10), Duration::from_secs(1))
        .single()
        .await?;
    year_dropdown.click().await?;
    println!("Clicked year dropdown");

    let year_1999_option = driver
        .find(By::XPath("//select[@id='SELECTOR_3']/option[@value='1999']"))
        .await?;
    year_1999_option.click().await?;
    println!("Selected '1999'");

    // Wait for the first "Suivant" button to become enabled
    loop {
        let first_suivant_button = driver
            .query(By::Css("[data-testid='ocfSignupNextLink']"))
            .single()
            .await?;

        let is_disabled: bool = driver.execute("return arguments[0].hasAttribute('disabled');", vec![first_suivant_button.to_json().expect("Failed to convert to JSON")]).await?.json().as_bool().unwrap_or(false);

        if !is_disabled {
            driver.execute("arguments[0].click();", vec![first_suivant_button.to_json().expect("Failed to convert to JSON")]).await?;
            println!("Clicked first 'Suivant' button");
            break;
        }

        println!("Waiting for the first 'Suivant' button to be enabled...");
        sleep(Duration::from_secs(1)).await;
    }

    // Wait for the second "Suivant" button to become enabled
    loop {
        let second_suivant_button = driver
            .query(By::XPath("//span[contains(@class, 'r-bcqeeo') and contains(@class, 'r-1ttztb7') and contains(@class, 'r-qvutc0') and contains(@class, 'r-poiln3') and text()='Suivant']"))
            .single()
            .await?;

        let is_disabled: bool = driver.execute("return arguments[0].hasAttribute('disabled');", vec![second_suivant_button.to_json().expect("Failed to convert to JSON")]).await?.json().as_bool().unwrap_or(false);

        if !is_disabled {
            driver.execute("arguments[0].click();", vec![second_suivant_button.to_json().expect("Failed to convert to JSON")]).await?;
            println!("Clicked second 'Suivant' button");
            break;
        }

        println!("Waiting for the second 'Suivant' button to be enabled...");
        sleep(Duration::from_secs(1)).await;
    }

    // Click the "Authentifier" button
    let captcha_id_elem = driver
    .query(By::XPath("/html/body/div/div/div[1]/p[2]"))
    .wait(Duration::from_secs(10), Duration::from_secs(1))
    .single()
    .await?;
    let captcha_id = captcha_id_elem.text().await?;
    println!("Captcha ID: {}", captcha_id);

    Ok(())
}
