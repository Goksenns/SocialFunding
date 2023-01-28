import Head from 'next/head'
import Header from '../components/header'
import Footer from '@/components/footer'
import HomePage from '@/components/disconnecting-page'
import { useMemo, useState,useEffect } from 'react'
import ProjectList from '../data/project'
import AddDonateModal from '../components/listing/donate-modal'
import AddVoteModal from '../components/listing/add-vote-modal'
import { useWallet } from '@solana/wallet-adapter-react'
import List from '@/components/listing/list'
import projectList from '../data/project'
import { useSocial } from '@/hooks/useSocial'

import { PublicKey } from "@solana/web3.js";

export default function Home() {

  const { createProject, initializeUser, transactionPending, donateProject, voteProject, loading, initialized, permission, donate, vote, setDonateProject, setVoteProject, setInitialized, setPermission } = useSocial()
  const { connected, publicKey } = useWallet()

  const [community, setCommunity] = useState([])
  const [projects, setProjects] = useState([])
  const [isCommMember, setIsCommMember] = useState(false)


  const [addDonateModalOpen, setAddDonateModalOpen] = useState(false)
  const [addVoteModalOpen, setAddVoteModalOpen] = useState(false)

const name = 'test'
const description = 'test'
  const toggleDonateModal = (value) => {
    console.log(value, 'value')
    setAddDonateModalOpen(true)
}

const toggleVoteModal = (value) => {

  setAddVoteModalOpen(true)
}


function handleSubmit(e) {
  e.preventDefault()

  console.log(amount),

  donatedProject(amount, account)
  setAddDonateModalOpen(false)
}

function handleSubmitVote(e) {
  e.preventDefault()

  console.log(vote),

  voteProject(vote, account)
  addVoteModalOpen(false)
}

const closeModal = () => {
  setAddDonateModalOpen(false)
  setAddVoteModalOpen(false)
}

  return (
    <div>
      <Head>
        <title>SocialFunding</title>
      </Head>
      <Header connected={connected} publicKey={publicKey} initialized = {initialized} transactionPending = {transactionPending} initializeUser={initializeUser}/>
      <main className="pt-10 pb-20">
        {connected ?
          <div>
            <button onClick={() => createProject({name, description})}>create Project</button>
          <List projects={projectList} toggleVoteModal={toggleVoteModal} toggleDonateModal={toggleDonateModal} connected={connected} addVoteModalOpen={addVoteModalOpen} addDonateModalOpen={addDonateModalOpen} handleSubmit={handleSubmit} handleSubmitVote={handleSubmitVote}/>
          <AddVoteModal addVoteModalOpen={addVoteModalOpen} handleSubmitVote={handleSubmitVote} closeModal={closeModal} />
          <AddDonateModal addDonateModalOpen={addDonateModalOpen} handleSubmit={handleSubmit} closeModal={closeModal} />
         </div> 
          :
          <div>
            <HomePage />
            {projects}
            <List projects={projectList} />
          </div>}
      </main>
      <Footer />
    </div>
  )
}