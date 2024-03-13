/********************************************************************************************************
 *  Autor: Fco. Javier Rodriguez Navarro								*
 *  WEB: www.pinguytaz.net										*
 *       https://github.com/pinguytaz									*
 *													*
 *  Descripción: CaroMail es un programa desarrollado en Rust con la intención de realizar 		*	
 *               un programa de practica en Rust y disponer de un comando para evio			*
 *               masivo de mails.									*
 *													*
 *  Uso:  caromal <fichero de configuración>								*
 *													*
 *  Historico:												*
 *      Creación:  0.1.0 Marzo 2024  FJRN								*
 *													*
 *  Librerias utilizadas:										*
 *      csv 		Para Leer fichero CSV (campos separados con ; de los mails)			*
 *      indicatif	Barra de progreso								*
 *      lettre		Envio de mails									*
 *      mime_guess	Tipos de ficheros que se adjuntan						*
 *      regex		Tratamiento de expresiones regulares para sustituir texto en nuestro caso	*
 *      rust-ini	Iterpretacion del fichero de configuración .INI					*
 ********************************************************************************************************/
use std::fs;
use std::fs::File;
use indicatif::{ProgressStyle};    // Barra de progreso
use std::thread;
use std::time::Duration;
use ini::Ini;                     // Lectura fichero de configuración
use regex::Regex;             // Expresiones regulares

// Librerias para el envio de mails
use lettre::{
    transport::smtp::{
        authentication::{Credentials, Mechanism},
        PoolConfig, 
        client::{Tls, TlsParameters},
    },
    Message, SmtpTransport, Transport,
    message::{
        header::ContentType,
       Attachment,MultiPart,SinglePart,
   },
};

// **************** Definicion de estructuras
/// Estructura que recoge los campos del fichero de datos: direccion email, nombre a sustituir en fichero.
struct Correo {
    	email: String,   
    	nombre: String,
	}

/// caromail <fichero de configuración>
fn main() {
    let prog = std::env::args().nth(0).expect("ERROR");
    let f_configura = std::env::args().nth(1).expect("**** ERROR **** debe introducir nombre fichero de configuración");
    
    banner();

    // Obtenemos configuración del fichero .ini
    let conf = Ini::load_from_file(&f_configura).unwrap();  // Recoge datos de .ini

    // Seccion de DATOS
    let datos = conf.section(Some("DATOS")).unwrap();
    let f_envios = datos.get("f_envios").unwrap();
    let pausa = datos.get("pausa").unwrap();
    let bloques = datos.get("bloques").unwrap();
    let pausabloque = datos.get("pausabloque").unwrap();

    // Seccion de MENSAJE
    let mensaje = conf.section(Some("MENSAJE")).unwrap();
    let asunto = mensaje.get("asunto").unwrap();
    let remitente = mensaje.get("remitente").unwrap();
    let f_texto = mensaje.get("f_texto").unwrap();
    let texto_plantilla = fs::read_to_string(f_texto).expect("ERROR Fichero contenido texto no disponible");
    let f_adjunto = mensaje.get("f_adjunto").unwrap();

    // Seccion de SMTP
    let smtp = conf.section(Some("SMTP")).unwrap();
    let identificacion = smtp.get("identificacion").unwrap();
    let smtp_server = smtp.get("smtp_server").unwrap();
    let port = smtp.get("port").unwrap();
    let usuario = smtp.get("usuario").unwrap();
    let clave = smtp.get("clave").unwrap();
   
    let (contador, datos) = lee_destinos(f_envios);      // Obtenemos los datos donde enviar los mails y el numero de registros capturados.

    println!("Programa: \"{}\"",prog);
    println!("Se procede a enviar {} mails",contador);
    println!("\tF. Configuracion: \"{}\"",&f_configura);
    println!("\tF. Datos: \"{}\"\n",&f_envios);
   
    // Definimos barra de progreso
    let sty = ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/red} {pos:>7}/{len:7} {msg}",)
              .unwrap().progress_chars("##-");
    let pb = indicatif::ProgressBar::new(contador);  // Definimos el numero de registros a enviar 
    pb.set_style(sty.clone());
    pb.set_message("completado envio");

    // Comenzamos el envio
    let mut i: u32 = 0;
    let re = Regex::new(r"<DESTINO>").unwrap();

    for registro in &datos {
        // Generamos texto a envia
        let texto = re.replace_all(&texto_plantilla, registro.nombre.to_string()); 

        // Montamos mensaje
        let email = cuerpo_mail(&registro.email, remitente, asunto, &texto, f_adjunto);
 
        // Abrimos conexion segun identificacion
        let mailer = match identificacion {
		        "TLS" => { let tls = TlsParameters::builder(smtp_server.to_owned()).dangerous_accept_invalid_certs(true).build();
                                   SmtpTransport::relay(smtp_server).unwrap() 
                                       .port(port.parse::<u16>().expect("Error conversion puerto"))
                                       .credentials(Credentials::new(usuario.to_string().to_owned(),clave.to_string().to_owned()))
                                       .authentication(vec![Mechanism::Plain])
                                       .pool_config(PoolConfig::new().max_size(20))
                                       .tls(Tls::Required(tls.expect("Error TLS")))
                                       .build() 
                                 },

		        "STARTTLS" => { let tls = TlsParameters::builder(smtp_server.to_owned()).dangerous_accept_invalid_certs(true).build();
                                       SmtpTransport::starttls_relay(smtp_server).unwrap() 
                                           .port(port.parse::<u16>().expect("Error conversion puerto"))
                                           .credentials(Credentials::new(usuario.to_string().to_owned(),clave.to_string().to_owned()))
                                           .authentication(vec![Mechanism::Plain])
                                           .pool_config(PoolConfig::new().max_size(20))
                                           .tls(Tls::Required(tls.expect("Error TLS")))
                                           .build() 
                                     },
                        "GMAIL" => { panic!("Tipo OAUTH2 para GMAIL no implementada") },
                        _ => { panic!("Tipo de identificacion erronea") }
		      };

        // Enviamos mail
        match mailer.send(&email){ 
            Ok(_) => {  pb.set_message(format!("{} \"{}\"",registro.email,registro.nombre));
                        pb.inc(1);
                     }
            Err(e) => panic!("No se ha podido enviar: {:?}", e), 
        }

        thread::sleep(Duration::from_millis(pausa.parse::<u64>().expect("Conversion pausa erronea")));  // Intervalo de envio entre correos
        if i == bloques.parse::<u32>().expect("Conversion erronea") {
            thread::sleep(Duration::from_millis(pausabloque.parse::<u64>().expect("Conversion pausa erronea")));  // Intervalo de envio entre correos
            i = 0;
        } else {
           i = i + 1;
        }
    };
    println!("\n\n **** COMPLETADO EL ENVIO **** \n\n");
}

///  Dibujo inicial
fn banner() {
    let s = format!(
        "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
        r#"                                                   ____                                 "#,
        r#"  ,----..                                        ,'  , `.                       ,--,    "#,
        r#" /   /   \                                    ,-+-,.' _ |              ,--,   ,--.'|    "#,
        r#"|   :     :             __  ,-.   ,---.    ,-+-. ;   , ||            ,--.'|   |  | :    "#,
        r#".   |  ;. /           ,' ,'/ /|  '   ,'\  ,--.'|'   |  ;|            |  |,    :  : '    "#,
        r#".   ; /--`   ,--.--.  '  | |' | /   /   ||   |  ,', |  ':  ,--.--.   `--'_    |  ' |    "#,
        r#";   | ;     /       \ |  |   ,'.   ; ,. :|   | /  | |  || /       \  ,' ,'|   '  | |    "#,
        r#"|   : |    .--.  .-. |'  :  /  '   | |: :'   | :  | :  |,.--.  .-. | '  | |   |  | :    "#,
        r#".   | '___  \__\/: . .|  | '   '   | .; :;   . |  ; |--'  \__\/: . . |  | :   '  : |__  "#,
        r#"'   ; : .'| ," .--.; |;  : |   |   :    ||   : |  | ,     ," .--.; | '  : |__ |  | '.'| "#,
        r#"'   | '/  :/  /  ,.  ||  , ;    \   \  / |   : '  |/     /  /  ,.  | |  | '.'|;  :    ; "#,
        r#"|   :    /;  :   .'   \---'      `----'  ;   | |`-'     ;  :   .'   \;  :    ;|  ,   /  "#,
        r#"\   \ .' |  ,     .-./                  |   ;/         |  ,     .-./|  ,   /  ---`-'   "#,
        r#"`---`    `--`---'                      '---'           `--`---'     ---`-'            "#,
    );
    println!("{}", s);
    
    let info = format!(
        "{}\n{}\n{}\n{}",
        r#"-----------------------------------------------"#,
        r#"| https://www.pinguytaz.net                   |"#,
        r#"| https://www.github.com/pinguytaz            |"#,
        r#"-----------------------------------------------"#
    );
    println!("{}", info);
}


///  Lee el fichero CSV y obtenene los mail y los nombres. 
 /*                                                                                     *
 * Parametro: fichero CSV                                                              *
 * Retorno:   Tupla con el numero de registros leeidos y el Vector estructura Correo   *
 ***************************************************************************************/
fn lee_destinos(f_envios: &str) -> (u64,Vec<Correo>) {
    
     let mut cont = 0;
     let fichero = File::open(f_envios.to_string()).expect("Error de apertuta fichero de DATOS");
     let mut rdr = csv::ReaderBuilder::new()
                   .has_headers(false)      // No tiene cabecera todo son datos
                   .delimiter(b';')        // Delimitador a;
                   .from_reader(fichero);
     let mut regs = Vec::new();
     for resultados in rdr.records() { 
         let registro = resultados.expect("ERROR registro CSV");
         let mail = Correo {
    	               email: registro[0].to_string(),
    	               nombre: registro[1].to_string(),
                    };
         regs.push(mail);
         cont = cont + 1;
     }

    return (cont,regs);
}

/// Forma el mail a enviar con su adjunto
fn cuerpo_mail(destino: &str, remitente: &str, asunto: &str, texto: &str, f_adjunto: &str) -> Message {
    let tipo = mime_guess::from_path(f_adjunto);

    let cuerpo_fichero = fs::read(f_adjunto).expect("ERROR Fichero adjunto no disponible");
    let tipo_contenido = ContentType::parse(tipo.first_or_octet_stream().as_ref()).unwrap();
    let attachment = Attachment::new(f_adjunto.to_string()).body(cuerpo_fichero, tipo_contenido);

    // Mensaje multiparte una con el texto plano y otra con el fichero que se adjunta.
    let email = Message::builder()
                .from(remitente.parse().expect("ERROR Remitente"))
                .to(destino.parse().expect("ERROR Destino"))
                .subject(asunto)
		.multipart(
		    MultiPart::mixed()
			.singlepart(SinglePart::builder()
                            .header(ContentType::TEXT_PLAIN)
			    .body(texto.to_string())
			)
			.singlepart(attachment)
		)
                .unwrap();

    return email;
}
