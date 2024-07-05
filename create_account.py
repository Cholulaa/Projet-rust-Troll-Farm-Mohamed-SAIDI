from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.common.keys import Keys
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC
import time

# Remplace ces valeurs par les informations nécessaires pour créer un compte
username = "your_username"
password = "your_password"
email = "your_email@example.com"
birth_month = "January"
birth_day = "1"
birth_year = "2000"

# Initialiser le WebDriver (assure-toi que le chemin vers ChromeDriver est correct)
chrome_options = webdriver.ChromeOptions()
# Configure the webdriver for your browser
driver_path = '/chromedriver.exe'
driver = webdriver.Chrome(executable_path=driver_path)


# Initialiser le WebDriver avec les options configurées
driver = webdriver.Chrome(service=chrome_service, options=chrome_options)
try:
    # Ouvrir la page d'inscription
    driver.get("https://x.com/i/flow/signup")
    time.sleep(2)  # Attendre que la page charge

    # Cliquer sur le bouton "Créer un compte"
    create_account_button = driver.find_element(By.XPATH, "//button[contains(., 'Créer un compte')]")
    create_account_button.click()
    time.sleep(2)

    # Remplir le formulaire d'inscription
    name_field = driver.find_element(By.NAME, "name")
    name_field.send_keys(username)

    email_field = driver.find_element(By.NAME, "email")
    email_field.send_keys(email)

    # Sélectionner la date de naissance
    month_dropdown = driver.find_element(By.ID, "SELECTOR_1")
    month_dropdown.send_keys(birth_month)

    day_dropdown = driver.find_element(By.ID, "SELECTOR_2")
    day_dropdown.send_keys(birth_day)

    year_dropdown = driver.find_element(By.ID, "SELECTOR_3")
    year_dropdown.send_keys(birth_year)

    # Cliquer sur le bouton "Suivant"
    next_button = driver.find_element(By.XPATH, "//span[text()='Suivant']")
    next_button.click()
    time.sleep(2)

    # Cliquer sur le bouton "Suivant" encore une fois
    next_button = driver.find_element(By.XPATH, "//span[text()='Suivant']")
    next_button.click()
    time.sleep(2)

    # Cliquer sur le bouton "S'inscrire"
    signup_button = driver.find_element(By.XPATH, "//span[text()='S'inscrire']")
    signup_button.click()
    time.sleep(2)

    # Remplir le mot de passe
    password_field = WebDriverWait(driver, 10).until(
        EC.presence_of_element_located((By.NAME, "password"))
    )
    password_field.send_keys(password)

    # Cliquer sur le bouton "Suivant" pour finaliser
    final_next_button = driver.find_element(By.XPATH, "//span[text()='Suivant']")
    final_next_button.click()

    # Attendre la fin du processus
    time.sleep(5)
    
finally:
    # Fermer le navigateur
    driver.quit()












/// let year_dropdown = driver
        .query(By::XPath("/html/body/div/div/div/div[2]/main/div/div/div/div[2]/div[2]/div[1]/div/div/div[2]/div[3]/div[3]/div/div[3]/select"))
        .wait(Duration::from_secs(10), Duration::from_secs(1))
        .single()
        .await?;
    let year_1999_option = year_dropdown
        .find(By::XPath("/html/body/div/div/div/div[2]/main/div/div/div/div[2]/div[2]/div[1]/div/div/div[2]/div[3]/div[3]/div/div[3]/select/option[27]"))
        .await?;
    year_1999_option.click().await?; // Select year "1999"