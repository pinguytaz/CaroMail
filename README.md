# CaroMail  (Caronte Mail)
<BR>
    Comando, programado en "Rust", para el envio masivo de mails.  
    El comando enviara mails a las direcciones definidas en el fichero de datos, junto al nombre de la persona de forma que los mails seran personalizados.  
    El mail constara de un texto personalizado y un fichero adjunto definido en el fichero de configuración.


Ejecución: **caromail &laquo; f_configuración &raquo;**  

**ficheros**  
  
* **f_configuración(.ini)**  (.ini) Fichero de configuración que se pasa por parametro.  
* **f_datos**    (.csv) Fichero con campos separados con ";" el primer campo es la direccion de correo y el segundo el nombre que se cambiara en el texto a enviar.  
* **f_texto**  (.txt) Fichero que contiene el texto y el campo "<DESTINO\>" se cambia por el segundo campo del ficherro de datos.  
* **f_adjunto**   Fichero que se adjunta al mail.  
<BR>

<CENTER>**Formato fichero de configuración(.ini)**</CENTER>  
<BR>
**[SMTP]**  Seccion definición de servidor donde enviar los mails.  
>>
**identificacion=** tipo de identificación (TLS o STARTTLS)  
**smtp_server=** Servidor SMTP  
**port=** Puerto  
**usuario=** usuario de la cuenta  
**clave=** Clave  

**[DATOS]**   Seción de datos define las pausa entre mensajes, asi como el fichero de mails a enviar.  
>>
**pausa=**  Pausa que se realiza entre cada envido de mail (tiempo en milisegundos)  
**bloques=** Numero de mensajes que se envian antes de realizar una pausa extra (numerico)  
**pausabloque=** pausa extra que se realiza cada "bloques" mensajes (tiempo en milisegundos)  
**f_envios=**    Fichero con las direcciones de mails a enviar (campo1) y el nombre (campo 2).  
  
**[MENSAJE]**    Seccion con los datos del mensaje.  
>>
**asunto=**   Asunto del mensaje.  
**remitente=** Dirección del remitente.  
**f_texto=**  Fichero que contiene el texto a enviar texto.txt donde ponga "<DESTINO\>" se cambiara por el sehundo campos.
**f_adjunto=** Nombre del fichero que se adjunta.  

<br><br>
__Website__: <https://www.pinguytaz.net>

