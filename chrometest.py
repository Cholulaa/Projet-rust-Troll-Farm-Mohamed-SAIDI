from selenium import webdriver

# Path to your ChromeDriver executable
# Escape backslashes in the file path
# Use raw string for the file path
driver_path = r"C:\Users\Mohamed\Documents\Cours-Bachelor-3-SI2\Rust\chromedriver.exe"


# Initialize Chrome WebDriver with options
chrome_options = webdriver.ChromeOptions()
chrome_options.add_argument("--start-maximized")  # Maximize the browser window on start

driver = webdriver.Chrome(executable_path=driver_path, chrome_options=chrome_options)

try:
    # Your automation code here
    driver.get("https://x.com/i/flow/signup")
    # Rest of your code
finally:
    # Close the browser
    driver.quit()
    