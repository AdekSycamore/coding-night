import Navbar from './components/Navbar';

interface x {
    title: string;
    imgPath: string;
    bodyMd: string;
    author: string;
    authorCat: string;
    authorDesc: string;
}

const PostPage = ({ title, imgPath, bodyMd, author, authorDesc, authorCat }: x) => {

    return (
        <div className="max-w-screen-lg mx-auto">
            <Navbar />
            <main className="mt-10">

                <div className="mb-4 md:mb-0 w-full mx-auto relative">
                    <div className="px-4 lg:px-0">
                        <h2 className="text-4xl font-semibold text-gray-800 leading-tight">
                            {title}
                        </h2>
                    </div>

                    <img src={imgPath} className="w-full object-cover lg:rounded" />
                </div>

                <div className="flex flex-col lg:flex-row lg:space-x-12">

                    <div className="px-4 lg:px-0 mt-12 text-gray-700 text-lg leading-relaxed w-full lg:w-3/4">
                        {bodyMd}
                    </div>

                    <div className="w-full lg:w-1/4 m-auto mt-12 max-w-screen-sm">
                        <div className="p-4 border-t border-b md:border md:rounded">
                            <div className="flex py-2">
                                <img src="https://www.kindpng.com/picc/m/678-6785842_my-account-account-icon-png-transparent-png.png"
                                    className="h-10 w-10 rounded-full mr-2 object-cover" />
                                <div>
                                    <p className="font-semibold text-gray-700 text-sm"> {author} </p>
                                </div>
                            </div>
                            <p className="text-gray-700 py-3">
                                {authorDesc}
                            </p>
                            <button className="px-2 py-1 text-gray-100 bg-green-700 flex w-full items-center justify-center rounded">
                                Conntact
                                <i className='bx bx-user-plus ml-2' ></i>
                            </button>
                        </div>
                    </div>
                </div>
                <iframe src="https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d82735.85602164213!2d20.62217218890335!3d49.60726255037349!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x473de53cb3f195cf%3A0x9865e0e4a3f03225!2s33-300%20Nowy%20S%C4%85cz!5e0!3m2!1spl!2spl!4v1701485641828!5m2!1spl!2spl" width="600" height="450" loading="lazy" ></iframe>
            </main>
        </div>
    );
}

export default PostPage;