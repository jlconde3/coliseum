const net = require('node:net');
const client = new net.Socket();

// Función para conectar con el servidor TCP
async function node_action(port, ip_address, request) {
    return new Promise((resolve, reject) => {
        client.connect(port, ip_address, () => {
            console.log('Conectado al servidor');
            client.write(JSON.stringify(request));
        });

        // Escuchar los datos recibidos del servidor
        client.on('data', (response) => {
            console.log(response);
            resolve(response); // Resolve the promise with the response
        });

        // Manejar errores de conexión
        client.on('error', (error) => {
            console.error('An error occurred: ' + error.message);
            reject(error); // Reject the promise with the error
        });

        // Manejar la conexión cerrada
        client.on('close', () => {
            console.log('Connection close with:' + ip_address + ":" + port);
        });
    });
}

async function create_item() {
    let nodeIp = document.getElementById("node_ip").value;
    let splitNodeIp = nodeIp.split(":");
    let ip_address = splitNodeIp[0];
    let port = splitNodeIp[1];

    
    let request = {
        entity: "CLIENT",
        action: "CREATE",
        data: document.getElementById("item_content").value
    };
    
    document.getElementById("item_content").value = "";
    
    try {
        const response = await node_action(port, ip_address, request);
        document.getElementById("create_response").value = response;
    } catch (error) {
        console.error('Error creating item:', error);
        // Handle error here, such as displaying an error message to the user
    }
}


async function retrieve_item() {

    let nodeIp = document.getElementById("node_ip").value;
    let splitNodeIp = nodeIp.split(":");
    let ip_address = splitNodeIp[0];
    let port = splitNodeIp[1];

    
    let request = {
        entity: "CLIENT",
        action: "RETRIEVE",
        data: document.getElementById("item_id").value
    };
    
    document.getElementById("item_id").value = "";
    
    try {
        const response = await node_action(port, ip_address, request);
        document.getElementById("retrieve_response").value = response;
    } catch (error) {
        console.error('Error retrieving item:', error);
        // Handle error here, such as displaying an error message to the user
    }
}

// Llamar a la función para conectar con el servidor cuando se pulsa el botón
document.getElementById('create_item').addEventListener('click', create_item);
document.getElementById('retrieve_item').addEventListener('click', retrieve_item);

