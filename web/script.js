// Placeholder for Solana connection
let connection;
let programId;

// Connect to Solana network (replace with actual network and program ID)
async function connectToSolana() {
    connection = new solanaWeb3.Connection("https://api.devnet.solana.com");
    programId = new solanaWeb3.PublicKey("YOUR_PROGRAM_ID_HERE");
}

// Initialize hardware list
async function initializeHardwareList() {
    const hardwareList = document.getElementById('hardware-list');
    const dummyHardware = [
        { id: "GPU001", name: "NVIDIA RTX 3080", price: "25 SOL/hour" },
        { id: "CPU001", name: "Intel i9-12900K", price: "10 SOL/hour" },
        { id: "SSD001", name: "1TB NVMe SSD", price: "5 SOL/hour" }
    ];

    dummyHardware.forEach(hw => {
        const hwElement = document.createElement('div');
        hwElement.className = 'hardware-item';
        hwElement.innerHTML = `
            <h3>${hw.name}</h3>
            <p>ID: ${hw.id}</p>
            <p>Price: ${hw.price}</p>
        `;
        hardwareList.appendChild(hwElement);
    });
}

// Handle form submission
document.getElementById('rent-form').addEventListener('submit', async (e) => {
    e.preventDefault();
    const hardwareId = document.getElementById('hardware-id').value;
    const rentalDuration = document.getElementById('rental-duration').value;

    try {
        // Here you would typically interact with your Solana program
        console.log(`Renting hardware ${hardwareId} for ${rentalDuration} hours`);
        // Placeholder for actual Solana transaction
        alert("Rental initiated! Check the console for details.");
    } catch (error) {
        console.error("Error initiating rental:", error);
        alert("Error initiating rental. Please check the console for details.");
    }
});

// Initialize the app
async function init() {
    await connectToSolana();
    await initializeHardwareList();
}

init();