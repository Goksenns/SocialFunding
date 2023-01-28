import { GlobeAmericasIcon, Bars3Icon, UserCircleIcon, MagnifyingGlassIcon } from '@heroicons/react/24/solid'
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui'
import Image from 'next/image'
import { truncate } from '@/utills/string'
require('@solana/wallet-adapter-react-ui/styles.css')
import ProjectListBox from './project-list-box';

function Header({connected, publicKey, initializeUser , initialized, transactionPending}) {
    return (
        <header className="sticky top-0 transition-all md:grid md:grid-cols-3 items-center px-10 xl:px-20 py-4 z-50 bg-white border-b">

            <div className="flex xl:justify-between px-6 transition-all duration-300">
                <button className="flex-1 flex items-center justify-left border rounded-full p-2  shadow-sm hover:shadow-md transition-all">
                    <input className="flex-1 text-gray-600 bg-transparent text-md font-light px-2" placeholder="Search .." />
                    <MagnifyingGlassIcon className="h-8 w-8 bg-[#FF385C] text-white stroke-[3.5px] p-2 rounded-full" style={{height:36, width:36, color:'#1f2937'}}/>
                </button>
            </div>

            <div className="flex items-center">
                <div className="text-gray-800 bg-transparent text-sm font-medium px-4">
                    <ProjectListBox/>
                </div>
                <WalletMultiButton className="phantom-button" startIcon={<UserCircleIcon style={{height:32, width:32, color:'#1f2937'}}/>}>
                    <span className="text-md font-medium text-black">{connected ? truncate(publicKey.toString()) : "Connect Wallet"}</span>
                </WalletMultiButton>
            </div>
            {initialized ? (<></>) : (<button type="button" className="border border-transparent cursor-pointer hover:bg-gray-100 rounded-full px-3 py-2" onClick={() => initializeUser()} disabled = {transactionPending}>
                        Initialize
                    </button>)}  
        </header>
    )
}

export default Header