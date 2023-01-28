import Image from 'next/image';

function HomePage() {
    return (
        <div className="">
            <h2>Social Funding ...</h2>
            <p>Connect your wallet to start</p>
            <img src="/world-map.svg" alt="wallet" />
        </div>
    )
}

export default HomePage