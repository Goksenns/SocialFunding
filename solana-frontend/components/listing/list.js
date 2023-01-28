import ListItem from './list-item'

function Listings({connected, showCommListing, projects, toggleVoteModal, toggleDonateModal, addVoteModalOpen, addDonateModalOpen, handleSubmit, handleSubmitVote}) {
    return (
        <div className="px-20">
            <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-10">
                {projects.map((item) => (
                    <ListItem key={item.id} {...item} publickey = {item.publicKey}  showCommListing={showCommListing} toggleVoteModal={toggleVoteModal} toggleDonateModal={toggleDonateModal} connected={connected} handleSubmit={handleSubmit} handleSubmitVote={handleSubmitVote} />
                ))}
            </div>
        </div>
    )
}

export default Listings