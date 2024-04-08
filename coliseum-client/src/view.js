const net = require('node:net');
const client = new net.Socket();

// Función para conectar con el servidor TCP
async function client_request(port, ip_address, request) {
    return new Promise((resolve, reject) => {
        client.connect(port, ip_address, () => {
            console.log('Conectado al servidor');
            client.write(JSON.stringify(request));
        });

        // Escuchar los datos recibidos del servidor
        client.on('data', (response) => {
            console.log(response);
            client.end(); // Cerrar la conexión después de recibir la respuesta
            resolve(response); // Resolve the promise with the response
        });

        // Manejar errores de conexión
        client.on('error', (error) => {
            console.error('An error occurred: ' + error.message);
            client.end(); // Cerrar la conexión en caso de error
            reject(error); // Reject the promise with the error
        });

        // Manejar la conexión cerrada
        client.on('close', () => {
            console.log('Connection closed with:' + ip_address + ":" + port);
        });
    });
}

function display_transactions(transactions) {
    // Parseamos la cadena JSON para obtener un array de transacciones
    var transactions_array = JSON.parse(transactions);

    // Iteramos sobre cada transacción
    transactions_array.forEach(transaction => {
        // Aquí puedes acceder a cada propiedad de la transacción
        let from = transaction.from;
        let to = transaction.to;
        let amount = transaction.amount;
        let date = transaction.date;
        let status = transaction.status;
        let comments = transaction.comments ? transaction.comments : ''; // Verificar si comments existe
        let table = document.querySelector("#transactions table tbody");
        let newRow = table.insertRow();

        newRow.innerHTML = `<td>${from}</td>
                            <td>${to}</td>
                            <td>${amount}</td>
                            <td>${date}</td>
                            <td>${status}</td>
                            <td>${comments}</td>`;
    });
}

async function get_account_information() {
    let nodeIp = document.getElementById("node_ip").value;
    let splitNodeIp = nodeIp.split(":");
    let ip_address = splitNodeIp[0];
    let port = splitNodeIp[1];
    let userName = document.getElementById("user_name").value;

    let request = {
        entity: "CLIENT",
        action: "GET_ACCOUNT",
        data: { user_name: userName }
    };

    try {
        const response = JSON.parse(await client_request(port, ip_address, request));
        const data = JSON.parse(response.data);
        document.getElementById("account_id").value = data.id;
        document.getElementById("account_balance").value = data.balance;
        display_transactions(data.transactions);
    } catch (error) {
        console.error('Error:', error);
    }
}

async function create_transaction() {
    let nodeIp = document.getElementById("node_ip").value;
    let splitNodeIp = nodeIp.split(":");
    let ip_address = splitNodeIp[0];
    let port = splitNodeIp[1];

    let account_id = document.getElementById("account_id").value;
    let transaction_from = document.getElementById("user_name").value;
    let transaction_to = document.getElementById("transaction_to").value;
    let transaction_amount = document.getElementById("transaction_amount").value;

    let data = { // Utiliza let para declarar la variable data
        account_id: account_id,
        from: transaction_from,
        to: transaction_to,
        amount: transaction_amount
    };

    let request = {
        entity: "CLIENT",
        action: "POST_TRANSACTION",
        data: data
    };

    try {
        const response = JSON.parse(await client_request(port, ip_address, request));
        console.log(response);
    } catch (error) {
        console.error('Error:', error);
    }
}

document.getElementById("log_in_botton").addEventListener("click", get_account_information);
document.getElementById("transaction_botton").addEventListener("click", create_transaction);
