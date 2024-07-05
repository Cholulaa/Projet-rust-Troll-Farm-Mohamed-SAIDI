use thirtyfour::prelude::*;
use tokio;
use std::time::Duration;
use std::error::Error;

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
    // fin du clique du bouton

    // Remplissage du champ "Nom et prénom" 
    let elem_nom_prenom = driver.find(By::XPath("/html/body/div/div/div/div[1]/div[2]/div/div/div/div/div/div[2]/div[2]/div/div/div[2]/div[2]/div[1]/div/div[2]/div[1]/label/div/div[2]/div/input")).await?; // trouve le champ "nom et prénom" avec le xpath
    elem_nom_prenom.send_keys("Test gros bg").await?;
    // fin du remplissage du champ "nom et prénom"

    // Remplissage du champ "Email"
    let elem_mail = driver.find(By::XPath("/html/body/div/div/div/div[1]/div[2]/div/div/div/div/div/div[2]/div[2]/div/div/div[2]/div[2]/div[1]/div/div[2]/div[2]/label/div/div[2]/div/input")).await?; // trouve le champ "nom et prénom" avec le xpath
    elem_mail.send_keys("grosb@zouz.com").await?;
    // fin du remplissage du champ "Email"

    // Sélection du mois de naissance
    let month_dropdown = driver
        .query(By::XPath("//select[@id='SELECTOR_1']"))
        .wait(Duration::from_secs(10), Duration::from_secs(1))
        .single()
        .await?;
    month_dropdown.click().await?;
    let march_option = driver
        .find(By::XPath("//select[@id='SELECTOR_1']/option[@value='3']"))
        .await?;
    march_option.click().await?;

    // Sélection du jour de naissance
    let day_dropdown = driver
        .query(By::XPath("//select[@id='SELECTOR_2']"))
        .wait(Duration::from_secs(10), Duration::from_secs(1))
        .single()
        .await?;
    day_dropdown.click().await?;
    let day_02_option = driver
        .find(By::XPath("//select[@id='SELECTOR_2']/option[@value='2']"))
        .await?;
    day_02_option.click().await?;

    // Sélection de l'année de naissance
    let year_dropdown = driver
        .query(By::XPath("//select[@id='SELECTOR_3']"))
        .wait(Duration::from_secs(10), Duration::from_secs(1))
        .single()
        .await?;
    year_dropdown.click().await?;
    let year_1999_option = driver
        .find(By::XPath("//select[@id='SELECTOR_3']/option[@value='1999']"))
        .await?;
    year_1999_option.click().await?;

    // Clique du bouton en dehors du cadre
    let cadre = driver // crée un élément button 
        .query(By::XPath("/html/body/div/div/div/div[2]/main/div/div/div/div[2]/div[2]")) // trouve le bouton avec le XPATH
        .wait(Duration::from_secs(5), Duration::from_secs(1)) // temps d'attente pour que le bouton charge
        .single()
        .await?;
    cadre.click().await?;

    let first_suivant_button = driver
        .query(By::XPath("//span[text()='Suivant']/ancestor::div[@role='button']"))
        .wait(Duration::from_secs(10), Duration::from_secs(1))
        .single()
        .await?;
    first_suivant_button.click().await?;

// Attendre que le deuxième bouton "Suivant" soit visible et cliquer dessus
    let second_suivant_button = driver
        .query(By::XPath("//span[text()='Suivant']/ancestor::div[@role='button']"))
        .wait(Duration::from_secs(10), Duration::from_secs(1))
        .single()
        .await?;
    second_suivant_button.click().await?;

    Ok(())

}
