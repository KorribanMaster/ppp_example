# README

## Notes

### How to setup GPRS with SIM868

````
AT+CPIN?
+CPIN: READY
OK

+CMTI: "SM",2
AT+CIPSHUT
SHUT OK
AT+CSTT="internet.telekom","congstar","cs"
OK
AT+CIICR
OK
AT+CIPSTATUS
OK

STATE: IP GPRSACT
AT+CIFSR
10.217.195.53
AT+CIPSTATUS
OK

STATE: IP STATUS

````

Now connect wit ppp

````

````

