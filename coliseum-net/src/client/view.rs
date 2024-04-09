
use std::net::TcpStream;

pub enum Endpoint {
    GetAccount,       // Recupera la información de la cuenta y de las transacciones asociadas a la cuenta
    CreateAccount,    // Crea una cuenta nueva
    CreateTransaction // Crea una nueva transacción y la chequea
}

pub struct CreateAccountData {
    username: String,
}

pub struct GetAccountData {
    account_id: String,
}

pub struct CreateTransactionData {
    from_id: String,
    to_id: String,
    amount: f64,
}

pub enum RequestData {
    None,
    CreateAccountData,
    GetAccountData,
    CreateTransactionData
}

pub struct Request {
    endpoint: Endpoint,
    orgin_addr: String,
    target_addr: String,
    data: RequestData,
}

struct View {}

impl View {
    pub fn handle_connection(stream: &mut TcpStream, request: Request) {
        match request.endpoint {

            Endpoint::CreateAccount => {
                if let Some(data) = request.data{
                    // Aquí maneja la creación de una nueva cuenta con los datos proporcionados en `data`
                } else {
                    // Manejar el caso donde no hay datos para crear una cuenta
                }
            }

            Endpoint::GetAccount => {
                if let Some(data) = request.data{
                    // Aquí maneja la solicitud para obtener información de la cuenta con los datos proporcionados en `data`
                } else {
                    // Manejar el caso donde no hay datos para obtener información de la cuenta
                }
            }

            Endpoint::CreateTransaction => {
                if let Some(data) = request.data{
                    // Aquí maneja la creación de una nueva transacción con los datos proporcionados en `data`
                } else {
                    // Manejar el caso donde no hay datos para crear una transacción
                }
            }
        }
    }
}

