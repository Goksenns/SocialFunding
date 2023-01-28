import { StarIcon } from '@heroicons/react/20/solid'
import { PencilIcon, TrashIcon, HeartIcon } from '@heroicons/react/24/outline'

function ListingItem({ idx, isCommMember, publickey, connected, subject, showDonatedListing, date, community, name, creator, vote, image, isDonated, showCommListing, toggleDonateModal, toggleVoteModal, description, handleSubmit, handleSubmitVote }) {

    return (
        <div>
            <div className="relative h-32 w-auto">
                {connected ? (
                    <>
                        <div className="cursor-pointer opacity-200 hover:opacity-100 transition-all duration-150  top-4 right-4 flex space-x-2">
                            <HeartIcon onClick={() => toggleVoteModal()} className="w-12 h-12 text-white opacity-80" style={{ height: 36, width: 36, color: '#1f2937' }} />
                            
                        </div>
                        <div className="cursor-pointer opacity-200 hover:opacity-100 transition-all duration-150  top-4 right-4 flex space-x-2">
                            <PencilIcon onClick={() => toggleDonateModal()} style={{ height: 36, width: 36, color: '#1f2937' }} className="w-12 h-12 text-white" />
                        </div>
                    </>
                ) : (
                    <></>
                )}
            </div>

            <div className="flex-1 justify-between items-center">
                <p className="text-sm text-gray-800">
                    {name}, {creator}, {community}
                </p>
                <p className='text-sm text-gray-800'>
                    {subject}
                </p>
            <p>{description}</p>

                <div className="flex items-center space-x-1">
                    <StarIcon className="h-3 w-3" />
                    <p className="text-sm text-gray-800">{4.8}</p>
                </div>
            </div>

            {showCommListing && <p className="text-sm font-light text-gray-600">{date}</p>}
        </div>
    )
}

export default ListingItem