import time
from io import BytesIO
import requests
from PIL import Image

from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.firefox.options import Options

def main():
    # you can run test on my website https://slanja.github.io/
    web = input("Enter website: ")

    # setting up selenium
    options = Options()
    options.headless = True

    driver = webdriver.Firefox(options=options)
    driver.get(web)
    time.sleep(0.2)

    # finding all <img> tags
    images = driver.find_elements(By.TAG_NAME, 'img')


    # name of the image will be number, so it's sorted and easier to go through
    filename = 0

    # going through every image
    for img in images:
        # getting source of an image
        src = img.get_attribute('src')

        try:
            response = requests.get(src)
            response.raise_for_status()

            # checking if the source is an image
            image = Image.open(BytesIO(response.content))
            # getting image type
            extension = image.format.lower()

            # downloading image to folder images/ and saving it as number.extension
            with open(f"images/{filename}.{extension}", 'wb') as f:
                f.write(response.content)

            print(f"Success {filename}.{extension} was downloaded")

            filename += 1

        except:
            print("Impostor ejected!")

    driver.close()


if __name__ == '__main__':
    main()