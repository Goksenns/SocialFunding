import * as anchor from '@project-serum/anchor'
import { useEffect, useMemo, useState } from 'react'
import toast from 'react-hot-toast'
import { SOCIAL_FUNDING_PROGRAM_PUBKEY } from '../constants'
import SfIDL from '../constants/social-funding.json'
import { SystemProgram, Keypair } from '@solana/web3.js'
import { utf8 } from '@project-serum/anchor/dist/cjs/utils/bytes'
import { findProgramAddressSync } from '@project-serum/anchor/dist/cjs/utils/pubkey'
import { useAnchorWallet, useConnection, useWallet } from '@solana/wallet-adapter-react'
import { authorFilter } from '@/utills'
import { PublicKey } from '@solana/web3.js'
import { set } from 'date-fns'
import { tr } from 'date-fns/locale'
import projectData from '../data/project'
import project from '../data/project'
import { Program, web3 } from '@project-serum/anchor';

export function useSocial() {
    const {connection} = useConnection()
    const {publicKey, sendTransaction} = useWallet()
    const anchorWallet = useAnchorWallet()

    const [permission, setPermission] = useState()
    const [projects, setProjects] = useState([])
    const [loading, setLoading] = useState(false)
    const [transactionPending, setTransactionPending] = useState(false)
    const [donateProject, setDonateProject] = useState({})
    const [voteProject, setVoteProject] = useState({})
    const [community, setCommunity] = useState([])
    const [isCommMember, setIsCommMember] = useState(false)
    const [initialized, setInitialized] = useState(false)

   
    const program = useMemo(() => {
        if (anchorWallet) {
            const provider = new anchor.AnchorProvider(connection, anchorWallet, anchor.AnchorProvider.defaultOptions())
            return new anchor.Program(SfIDL, SOCIAL_FUNDING_PROGRAM_PUBKEY, provider)
        }
    }, [connection, anchorWallet])

    useEffect(() => {
        const start = async () => {
            if (program && publicKey && !transactionPending) {
                try {
                    console.log(program.methods, 'lalalla')
                    const [profilePda, profileBump] = await findProgramAddressSync(publicKey.toBuffer(), program.programId)
                    const profileAccount = await program.account.stage.fetch(profilePda)
                    if(profileAccount){
                        setInitialized(true)
                        setLoading(true)
                        const community = await program.account.community.all()
                        setCommunity(community)
                        console.log(community, 'community')
                        const projects = await program.account.project.all()
                        setProjects(projects)
                        console.log(projects, 'projects')
                    } else {
                        setInitialized(false)
                    }
                } catch (error) {
                    console.log(error)
                    setInitialized(false)
                } finally {
                    setLoading(false)
                }
            }
        }
        start()
    }, [program, publicKey, transactionPending])

    const initializeUser = async () => {
        if (program && publicKey) {
            try {
                setTransactionPending(true)
                setLoading(true)
                const [profilePda] = findProgramAddressSync([publicKey.toBuffer()], program.programId)

                const tx = await program.methods
                    .stage()
                    .accounts({
                        solBank: "4jb5cDEuM1FMzd2GAgmYagNswgkYgMeUpbfA6v1Wessa",
                        admin: "BsieGe4MRQoTKQKYw5KEcSiWM2PwEmF43DrZJTTj9XAB",
                        management: "BsieGe4MRQoTKQKYw5KEcSiWM2PwEmF43DrZJTTj9XAB",
                        systemProgram: SystemProgram.programId,
                    })
                    .rpc()
                setInitialized(true)
                console.log('lalalla')
                toast.success('Successfully initialized user.')
            } catch (error) {
                console.log(error)
            } finally {
                setLoading(false)
                setTransactionPending(false)
            }
        }
    }

    // const createProject = async ({name, description}) => {
    //     console.log(name, description, "'name, description")
    //     if (program && publicKey) { 
    //         setTransactionPending(true)
    //         setLoading(true)
    //         try {
    //             const [profilePda] = await findProgramAddressSync(publicKey.toBuffer(), program.programId)
    //             console.log(program.methods, 'lalalla')
    //             console.log(profilePda, 'profilePda')
    //             await program.methods.createProject(name, description)
    //             .accounts({
    //                 project: program.account.project.publicKey,
    //                 admin: "BsieGe4MRQoTKQKYw5KEcSiWM2PwEmF43DrZJTTj9XAB",
    //                 management: "BsieGe4MRQoTKQKYw5KEcSiWM2PwEmF43DrZJTTj9XAB",
    //                 community: "BsieGe4MRQoTKQKYw5KEcSiWM2PwEmF43DrZJTTj9XAB",
    //                 counter: 1,
    //                 creator: "BsieGe4MRQoTKQKYw5KEcSiWM2PwEmF43DrZJTTj9XAB",
    //                 systemProgram: SystemProgram.programId,
    //             }).rpc()
    //             toast.success('Successfully created project.')
    //         } catch (error) {
    //             console.log(error)
    //         } finally {
    //             setLoading(false)
    //             setTransactionPending(false)
    //         }
    //     }
    // }


    const getProject = async() => {
      try {
        const projectData = await program.account.project.all()
        setProjects(projectData)
        console.log(projectData, 'projectData')

    } catch (error) {
        console.log("Error in getProject: ", error)
        setProjects(null);
    }
}



    const createProject = async ({name, description}) => {
        if (program && publicKey) {
        try {
          await program.methods.createProject(name,description, {
            accounts: {
                project: publicKey,
                admin: publicKey,
                management: "BsieGe4MRQoTKQKYw5KEcSiWM2PwEmF43DrZJTTj9XAB",
                community: "BsieGe4MRQoTKQKYw5KEcSiWM2PwEmF43DrZJTTj9XAB",
                counter: 1,
                creator: "BsieGe4MRQoTKQKYw5KEcSiWM2PwEmF43DrZJTTj9XAB",
                systemProgram: SystemProgram.programId,
            }
            })
          console.log("successfully created project")
          console.log(program.methods, 'lalalla 2')
          await getProject(project);
        } catch (error) {
          console.log("Error create project", error)
        }
    }   
      };

    
      const distributeFund = async () => {
        if (program && publicKey) {
            try {
                setTransactionPending(true)
                setLoading(true)
                const [profilePda] = findProgramAddressSync([publicKey.toBuffer()], program.programId)
                console.log(profilePda, 'profilePda')
                const tx = await program.methods
                    .stage()
                    .accounts({
                        solBank: "4jb5cDEuM1FMzd2GAgmYagNswgkYgMeUpbfA6v1Wessa",
                        admin: "BsieGe4MRQoTKQKYw5KEcSiWM2PwEmF43DrZJTTj9XAB",
                        management: "BsieGe4MRQoTKQKYw5KEcSiWM2PwEmF43DrZJTTj9XAB",
                        systemProgram: SystemProgram.programId,
                    })
                    .rpc()
                setInitialized(true)
                console.log('lalalla')
                toast.success('Successfully initialized user.')
            } catch (error) {
                console.log(error)
            } finally {
                setLoading(false)
                setTransactionPending(false)
            }
        }
    }
 




    return {initializeUser, createProject, transactionPending, initialized }
}
