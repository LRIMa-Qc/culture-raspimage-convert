# Required material
![[Pasted image 20260626150452.png]]
- MicroSD card
- MicroSD to USB adapter
- Bluetooth USB adapter
- Raspberry pi 4/5
- Power Supply for raspberry pi (USB-C)
- Raspberry pi casing

# Download LRIMa central image
From within a greenhouse, select connected devices.
![[Pasted image 20260626113148.png]]
click on it.


> [!CRITICAL]
> THIS PROGRAM ONLY WORKS ON CHROMIUM BROWSERS
There should be a "download Image" button at the top of the screen.


![[Pasted image 20260626113252.png]]
Upon press, the following page will open. 
![[Pasted image 20260626113548.png]]
Fill the form with the required information. 
## Object ID:
Can be obtained by copying the gray text besides the name of the central
![[Pasted image 20260626114029.png]]

## Auth token
Can be obtained by clicking the "copy Auth token" in the website
![[Pasted image 20260626114203.png]]

> [!INFO]
> This has not been tested with 5Ghz wifi. For the sake of stability, please stay on 2.4Ghz wifi.
## WIFI SSID
Enter your wifi name

## WIFI Password
Enter your wifi password

then, press the `Download` button.

> [!WARNING]
> Please ensure the file is downloaded in one shot. If there is any interruption, assume the file is corrupted.

![[image.png]]
the download will then start!

# Flashing the Image file
First, download balena etcher. 
https://etcher.balena.io/

Then, open it.
![[Pasted image 20260626143909.png]]

first, select `flash from file`
![[flash_button.png]]
If the MicroSD card reader has not been already inserted, insert it now. 

you will see it in the second column
![[Pasted image 20260626144749.png]]
if not selected automatically, press change and select your drive.
![[Pasted image 20260626144845.png]]

finally, press flash
![[Pasted image 20260626144901.png]]
then, wait...
until balena etcher shows that it finished the flash.
![[Pasted image 20260626144926.png]]

Take the MicroSD card, then put it in the raspberry pi. 

![[Pasted image 20260626150632.png]]
then the bluetooth adaptator
![[Pasted image 20260626151134.png]]
then power on the raspberry pi!

wait for around 15 minutes...
![[Pasted image 20260626153424.png]]
And it should be good!

