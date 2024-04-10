const net = require('node:net');
const client = new net.Socket();

async function client_request(port, ip_address, request) {
    return new Promise((resolve, reject) => {
        client.connect(port, ip_address, () => {
            console.log('Conectado al servidor');
            client.write(JSON.stringify(request));
        });

        client.on('data', (response) => {
            resolve(response.toString());
            client.end();
        });

        client.on('error', (error) => {
            console.error('An error occurred: ' + error.message);
            client.end();
            reject(error);
        });

        client.on('close', () => {
            console.log('Connection closed with:' + ip_address + ":" + port);
        });
    });
}

function display_transactions(transactions) {

    var transactions_array = JSON.parse(transactions);


    transactions_array.forEach(transaction => {

        let from = transaction.from;
        let to = transaction.to;
        let amount = transaction.amount;
        let date = transaction.date;
        let status = transaction.status;

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
    let accountId = document.getElementById("account_id").value;

    let request = {
        endpoint: "GetAccount",
        origin_addr: "localhost",
        target_addr: "127.0.0.1:5000",
        data: JSON.stringify({ account_id:accountId })
    };

    try {
        const response = JSON.parse(await client_request(port, ip_address, request));
        const data = JSON.parse(response.data);
        console.log(data)

        document.getElementById("account_balance").value = data.balance;
        //display_transactions(data.transactions);
    } catch (error) {
        console.error('Error:', error);
    }
}

async function create_transaction() {
    let nodeIp = document.getElementById("node_ip").value;
    let splitNodeIp = nodeIp.split(":");
    let ip_address = splitNodeIp[0];
    let port = splitNodeIp[1];

    let from_id = document.getElementById("account_id").value;
    let to_id = document.getElementById("to_id").value;
    let amount = document.getElementById("amount").value;

    let data = {
        from_id: from_id,
        to_id: to_id,
        amount: amount
    };

    let request = {
        endpoint: "CreateTransaction",
        origin_addr: "localhost",
        target_addr: "127.0.0.1:5000",
        data: JSON.stringify(data)
    };

    console.log(request);

    try {
        const response = JSON.parse(await client_request(port, ip_address, request));
        console.log(JSON.parse(response.data));
    } catch (error) {
        console.error('Error:', error);
    }
}

document.getElementById("log_in_botton").addEventListener("click", get_account_information);
document.getElementById("new_transaction").addEventListener("click", create_transaction);
