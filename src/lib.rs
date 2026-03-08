use anchor_lang::prelude::*;
// ID del Solana Program, este espacio se llena automaticamente al hacer "build"
declare_id!("HVq1Vb58wiuwxQfsVqWSTZpMMPq8a6HEJjJxxNJggCRn");

#[program] // Macro que convierte codigo de Rust a Solana.
pub mod tienda {
    use super::*; // Importa todas los structs y enums definidos fuera del modulo


//CREATE
    //////////////////////////// Instruccion: Crear Tienda /////////////////////////////////////
    /*
    Permite la creacion de una PDA (Program Derived Adress), un tipo especial de cuenta en solana que permite prescindir 
    del uso de llaves privadas para la firma de transacciones. 

    Esta cuenta contendra el objeto (struct) de tipo Tienda donde podremos almacenar los Articulos. 
    La creacion de la PDA depende de 3 cosas:
        * Wallet address 
        * Program ID 
        * string representativo, regularmente relacionado con el nombre del proyecto
    
    La explicacion de esto continua en el struct NuevaTienda

    Parametros de entrada:
        * nombre -> nombre de la tienda -> tipo string
     */
    pub fn crear_tienda(context: Context<NuevaTienda>, nombre: String) -> Result<()> {
        // "Context" siempre suele ir como primer parametro, ya que permite acceder al objeto o cuenta con el que queremos interactuar
        // Dentro del context va al tipo de objeto o cuenta con el que deseamos interactuar. 
        let owner_id = context.accounts.owner.key(); // Accedemos al wallet address del caller 
        msg!("Owner id: {}", owner_id); // Print de verificacion

        let articulos: Vec<Articulo> = Vec::new(); // Crea un vector vacio 

        // Creamos un Struct de tipo tienda y lo guardamos directamente 
        context.accounts.tienda.set_inner(Tienda { 
            owner: owner_id,
            nombre,
            articulos,
        });
        Ok(()) // Representa una transaccion exitosa 
    }


    //////////////////////////// Instruccion: Agregar Articulo /////////////////////////////////////
    /*
    Agrega un articulo al vector de articulos Contenido en el struct Tienda. 
    En este caso el contexto empleado es el struct NuevoArticulo. Mientras que NuevaTienda permite crear 
    Instancias de una Tienda. NuevoArticulo permite crear y modificar los valores relacionados a cualquier
    struct de tipo Articulo.

    Parametros de entrada:
        * nombre -> nombre del articulo -> string
        * cantidad -> numero de cantidad del articulo -> u16
     */ 
    pub fn agregar_articulo(context: Context<NuevoArticulo>, nombre: String, fecha: String, lugar: String, cantidad: u16, precio: u16) -> Result<()> {
        require!( // Medida de seguridad para identificar que SOLO el owner de la tienda sea el que hace cambios en ella
            context.accounts.tienda.owner == context.accounts.owner.key(), // Condicion, true -> continua, false -> error
            Errores::NoEresElOwner // Codigo de error, ver enum Errores
        ); 

        let articulo = Articulo { // Creacion de un struct tipo Articulo
            nombre,
            fecha,
            lugar,
            cantidad,
            precio,
            disponible: true,
        };

        context.accounts.tienda.articulos.push(articulo); // Agrega el Articulo al vector de articulos de Tienda

        Ok(()) // Transaccion exitosa
    }


//READ
    //////////////////////////// Instruccion: Ver Articulos /////////////////////////////////////
    /*
    Muestra en el log de la transaccion el contenido completo del vector de articulos de la Tienda

    Parametros de entrada:
        Ninguno
     */
    pub fn ver_articulos(context: Context<NuevoArticulo>) -> Result<()> {
        require!( // Medida de seguridad 
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );
        
        msg!("La tienda es: {}", context.accounts.tienda.nombre); // Print en log del nombre de la Tienda

        // :#? requiere que NuevoArticulo tenga atributo Debug. Permite la visualizacion completa del vector en el log
        msg!("La lista de articulos actualmente es: {:#?}", context.accounts.tienda.articulos); // Print en log

        Ok(()) // Transaccion exitosa 
    }


//UPDATE    


    //////////////////////////// Instruccion: cambiar Fecha /////////////////////////////////////
    /* 
    Cambia la fecha registrada

    Parametros de entrada:
        * nombre -> Nombre del articulo -> string
     */
    pub fn cambiar_fecha(context: Context<NuevoArticulo>, nombre: String, fecha: String) -> Result<()> {
        require!( // Medida de seguridad
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let articulos = &mut context.accounts.tienda.articulos; // Referencia mutable al vector de articulos
        for i in 0..articulos.len() { // Se itera mediante el indice el vector de articulos

            if articulos[i].nombre == nombre { // Si ecuentra el nombre del articulo procede a cambiar el valor 
                articulos[i].fecha = fecha;
                
                msg!("El articulo: {} ahora tiene una nueva fecha", nombre); // log print del nuevo cambio
                return Ok(()); // Transaccion exitosa
            }
        }

        Err(Errores::ArticuloNoExiste.into()) // Transaccion fallida, articulo no existe
    }


 //////////////////////////// Instruccion: cambiar Lugar /////////////////////////////////////
    /* 
    Cambia el lugar registrado

    Parametros de entrada:
        * nombre -> Nombre del articulo -> string
     */
    pub fn cambiar_lugar(context: Context<NuevoArticulo>, nombre: String, lugar: String) -> Result<()> {
        require!( // Medida de seguridad
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let articulos = &mut context.accounts.tienda.articulos; // Referencia mutable al vector de articulos
        for i in 0..articulos.len() { // Se itera mediante el indice el vector de articulos

            if articulos[i].nombre == nombre { // Si ecuentra el nombre del articulo procede a cambiar el valor 

                articulos[i].lugar = lugar;
                
                msg!("El articulo: {} ahora tiene un cambio de lugar", nombre); // log print del nuevo cambio
                return Ok(()); // Transaccion exitosa
            }
        }

        Err(Errores::ArticuloNoExiste.into()) // Transaccion fallida, articulo no existe
    }


 //////////////////////////// Instruccion: cambiar cantidad /////////////////////////////////////
    /* 
    Cambia la cantidad del articulo seleccionado

    Parametros de entrada:
        * nombre -> Nombre del articulo -> string
     */
    pub fn cambiar_cantidad(context: Context<NuevoArticulo>, nombre: String, cantidad: u16) -> Result<()> {
        require!( // Medida de seguridad
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let articulos = &mut context.accounts.tienda.articulos; // Referencia mutable al vector de articulos
        for i in 0..articulos.len() { // Se itera mediante el indice el vector de articulos

            if articulos[i].nombre == nombre { // Si ecuentra el nombre del articulo procede a cambiar el valor 

                articulos[i].cantidad = cantidad;
                
                msg!("El articulo: {} cambió su número de existencias", nombre); // log print del nuevo cambio
                return Ok(()); // Transaccion exitosa
            }
        }

        Err(Errores::ArticuloNoExiste.into()) // Transaccion fallida, articulo no existe
    }


 //////////////////////////// Instruccion: cambiar precio /////////////////////////////////////
    /* 
    Cambia el precio del articulo seleccionado

    Parametros de entrada:
        * nombre -> Nombre del articulo -> string
     */
    pub fn cambiar_precio(context: Context<NuevoArticulo>, nombre: String, precio: u16) -> Result<()> {
        require!( // Medida de seguridad
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let articulos = &mut context.accounts.tienda.articulos; // Referencia mutable al vector de articulos
        for i in 0..articulos.len() { // Se itera mediante el indice el vector de articulos

            if articulos[i].nombre == nombre { // Si ecuentra el nombre del articulo procede a cambiar el valor 

                articulos[i].precio = precio;
                
                msg!("El articulo: {} cambió su precio", nombre); // log print del nuevo cambio
                return Ok(()); // Transaccion exitosa
            }
        }

        Err(Errores::ArticuloNoExiste.into()) // Transaccion fallida, articulo no existe
    }



    //////////////////////////// Instruccion: Alternar Estado /////////////////////////////////////
    /* 
    Cambia el estado de disponible de false a true o de true a false.

    Parametros de entrada:
        * nombre -> Nombre del articulo -> string
     */
    pub fn alternar_estado(context: Context<NuevoArticulo>, nombre: String) -> Result<()> {
        require!( // Medida de seguridad
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let articulos = &mut context.accounts.tienda.articulos; // Referencia mutable al vector de articulos
        for i in 0..articulos.len() { // Se itera mediante el indice el vector de articulos
            let estado = articulos[i].disponible;  // Se almacena el estado del vector actual

            if articulos[i].nombre == nombre { // Si ecuentra el nombre del articulo procede a cambiar el valor del estado 
                let nuevo_estado = !estado;
                articulos[i].disponible = nuevo_estado;
                msg!("El articulo: {} ahora tiene un valor de disponibilidad: {}", nombre, nuevo_estado); // log print de la nueva disponibilidad
                return Ok(()); // Transaccion exitosa
            }
        }

        Err(Errores::ArticuloNoExiste.into()) // Transaccion fallida, articulo no existe
    }



//DELETE
    //////////////////////////// Instruccion: Eliminar Articulo /////////////////////////////////////

    /*
    Elimina un articulo apartir de su nombre. Error si articulo no existe, Error si vector vacio. 

    Parametros de entrada:
        * nombre -> Nombre del articulo -> string
     */
    pub fn eliminar_articulo(context: Context<NuevoArticulo>, nombre: String) -> Result<()> {
        require!( // Medida de seguridad
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let articulos = &mut context.accounts.tienda.articulos; // Referencia mutable al vector de articulos

        for i in 0..articulos.len() { // Se itera mediante el indice todo el contenido del vector en busca del articulo a eliminar
            if articulos[i].nombre == nombre { // Si lo encuentra prodece a borrarlo mediante el metodo remove
                articulos.remove(i);
                msg!("Articulo {} eliminado!", nombre); // Mensaje de borrado exitoso
                return Ok(()); // Transaccion exitosa
            }
        }
        Err(Errores::ArticuloNoExiste.into()) // Transaccion fallida, nunca encontro el articulo
    }

}



//ERRORES_______________________________________________________________________________
/*
Codigos de error
Todos los codigos se almacenan en un enum con la siguiente estructura:
#[msg("MENSAJE DE ERROR")] (dentro de las comillas)
NombreDelError, (En camel case)
*/
#[error_code]
pub enum Errores {
    #[msg("Error, no eres el propietario de la tienda que deseas modificar")]
    NoEresElOwner,
    #[msg("Error, el articulo con el que deseas interactuar no existe")]
    ArticuloNoExiste,
}

//STRUCT_______________________________________________________________________________
#[account] // Especifica que el struct es una cuenta que se almacenara en la blockchain
#[derive(InitSpace)] // Genera la constante INIT_SPACE y determina el espacio de almacenamiento necesario 
pub struct Tienda { // Define la Tienda
    owner: Pubkey, // Pubkey es un formato de llave publica de 32 bytes 

    #[max_len(60)] // Cantidad maxima de caracteres del string: nombre
    nombre: String,
    #[max_len(10)] // Tamaño maximo del vector articulos 
    articulos: Vec<Articulo>,
}

/*
Struct interno o secundario (No es una cuenta). Se define por derive y cuenta con los siguientes atributos:
    * AnchorSerialize -> Permite guardar el struct en la cuenta 
    * AnchorDeserialize -> Permite leer su contenido desde la cuenta 
    * Clone -> Para copiar su contenido o valores 
    * InitSpace -> Calcula el tamaño necesario para ser almacenado en la blockchain
    * PartialEq -> Para usar sus valores y compararlos con "=="
    * Debug -> Para mostrarlo en log con ":?" o ":#?"
*/
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Articulo {
    
    #[max_len(60)]  // Cantidad maxima de caracteres del string: nombre
    nombre: String,
     #[max_len(10)] // Cantidad maxima de caracteres del string: fecha
    fecha: String,
    #[max_len(60)]  // Cantidad maxima de caracteres del string: lugar
    lugar: String,
    
    // Los siguientes datos no rquieren de max_len porque ya estan definidos (numero de 16 bits y false o true)
    cantidad: u16, 
    precio: u16,
    disponible: bool,
}

//CONTEXT_______________________________________________________________________________
// Creacion de los contextos para las instrucciones (funciones)
#[derive(Accounts)] // Especifica que este struct describe las cuentas que se requieren para determinada instruccion
pub struct NuevaTienda<'info> { // contexto de la instruccion
    #[account(mut)] 
    pub owner: Signer<'info>, // Se define que el owner como el que pagara la transaccion, por eso es mut, para que cambie el balance de la cuenta

    //PDA
    #[account(
        init, // Indica que al llamar la instruccion se creara una cuenta
        // puede ser remplazado por "init_if_needed" para que solo se cree una vez por caller
        payer = owner, // Se especifica que quien paga el llamado a la instruccion, en este caso llama la instruccion 
        space = Tienda::INIT_SPACE + 8, // Se calcula el espacio requerido para almacenar el Solana Program On-Chain
        seeds = [b"tienda", owner.key().as_ref()], // Se especifica que la cuenta es una PDA que depende de un string y el id del owner
        bump // Metodo para determinar el el id de la tienda en base a lo anterior 
    )]
    pub tienda: Account<'info, Tienda>, // Se especifica que la cuenta creada (PDA) almacenara la tienda 

    pub system_program: Program<'info, System>, // Programa necesario para crear la cuenta 
}

// Contexto para la creacion y modificacion de articulos 
#[derive(Accounts)] // Especifica que este struct se requiere para todas las instrucciones relacionadas con la creacion o modificacion de Articulo
pub struct NuevoArticulo<'info> {
    pub owner: Signer<'info>, // El owner de la cuenta es quien paga la transaccion

    #[account(mut)] 
    pub tienda: Account<'info, Tienda>, // Se marca tienda como mutable porque se modificara tanto el vector como los articulos que contiene
}
