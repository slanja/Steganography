from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.firefox.options import Options

def main():
    web = input("Enter website: ")

    options = Options()
    options.headless = False

    driver = webdriver.Firefox(options=options)
    driver.get(web)

if __name__ == '__main__':
    main()