DEV=hci0

hciconfig $DEV up
hciconfig $DEV noleadv

hcitool -i $DEV cmd 0x08 0x0008 1a 03 03 AF DE 15 09 55 6E 69 74 20 30 30 30 30 20 44 6F 6F 72 20 4C 6F 63 6B 00
hcitool -i $DEV cmd 0x08 0x0006 A0 00 A0 00 03 00 00 00 00 00 00 00 00 07 00
hcitool -i $DEV cmd 0x08 0x000a 01

