import { useState, Fragment } from 'react'
import { Dialog, Transition } from '@headlessui/react'

function MyDialog({ addDonateModalOpen , handleSubmit, setAddDonateModalOpen, closeModal}) {
  let [amount, setAmount] = useState(0)

  function handleChange(e) {
    setAmount(e.target.value)
  }
 

  return (
    <Transition appear show={addDonateModalOpen} as={Fragment}>
      <Dialog as="div" className="relative z-50" onClose={closeModal}>
        <Transition.Child as={Fragment} enter="ease-out duration-300" enterFrom="opacity-0" enterTo="opacity-100" leave="ease-in duration-200" leaveFrom="opacity-100" leaveTo="opacity-0">
            <div className="fixed inset-0 bg-black bg-opacity-25" />
        </Transition.Child>
        <div className="fixed inset-0 overflow-y-auto">
          <div className="flex min-h-full items-center justify-center p-4 text-center">
              <Transition.Child as={Fragment} enter="ease-out duration-300" enterFrom="opacity-0 scale-95" enterTo="opacity-100 scale-100" leave="ease-in duration-200" leaveFrom="opacity-100 scale-100" leaveTo="opacity-0 scale-95">
                  <Dialog.Panel className="w-full max-w-md transform overflow-hidden rounded-2xl bg-white p-6 text-left align-middle shadow-xl transition-all">
                      <Dialog.Title as="h3" className="text-lg font-medium leading-6 text-gray-900">
                        Donate
                      </Dialog.Title>
                      <div className="mt-2">
                      <div className="grid grid-cols-1 gap-3">
                          <label className="flex flex-col border rounded-lg px-3 py-2" htmlFor="location">
                          <label htmlFor="amount" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">Donate Amount</label>
                          <input type="number" value={amount} onChange={handleChange} id="amount" className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" required />
                          </label>
                      </div>
                    
                      <div className="mt-4 flex justify-end">
                      <button type='submit' onClick={handleSubmit} className="border rounded-lg px-4 py-2 text-sm font-medium">
                            Donate
                        </button>
                        <button onClick={closeModal} type="button" className="border rounded-lg px-4 py-2 text-sm font-medium">
                            Cancel
                        </button>
                    </div>
                  </div>
                  </Dialog.Panel>
                        </Transition.Child>
                    </div>
                </div>
            </Dialog>
        </Transition>
  ) 
}

export default MyDialog

