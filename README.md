# Steganografie
Steganografie je věda, která se zabývá utajenou komunikací. Komunikace probíhá v takové formě, aby nebylo možné zjistit, že komunikace probíhá. Proto se ve steganografii zprávy skrývají do takzvaného **coveru**, to může být například obrázek, video nebo audio soubor.

## Least Significant Bit (LSB)
LSB je metoda steganografie, která vkládá zprávu do nejméně potřebných bitů, u obrázku například do pixelů, touto změnou lehce upravíme odstín barev, tuto změnu však lidské oko nepostřehne.

<center>
  <img src="https://miro.medium.com/v2/resize:fit:720/format:webp/1*Gu_RomzVTPMEJ1hfKanRBA.png">
  Obrázek 1: Reprezentace obrázku jako 2D pole RGB pixelů
</center>

# My tools
## LSB Encoder & Decoder
Nástroj napsaný v Rustu, který dokáže vložit tajnou zprávu do obrázku a zase zprávu přečíst.

### Demo
Nástroj upraví nejméně potřebné bity vstupního obrázku a vytvoří nový, který bude obsahovat naši zprávu.

<center>
  <img src="/LSB_Encoder_and_Decoder/images/crab.png">
  Obrázek 2: Vstupní obrázek
</center><br><br>  

Zde si můžete všimnout obrázku, který obsahuje tajnou zprávu, i když je upravena průhlednost barev, tak nelze poznat změnu od původního obrázku.

<center>
  <img src="/LSB_Encoder_and_Decoder/images/output.png">
  Obrázek 3: Obrázek obsahující zprávu
</center>

## Steganalysis
Jednoduchý script v pythonu, který dokáže z libovolného webu stáhnout veškeré obrázky. Pro samotnou analýzu jsem do "LSB Encoder & Decoder" přidal funkci, která prochází při čtení zpráv veškeré obrázky v dané složce.