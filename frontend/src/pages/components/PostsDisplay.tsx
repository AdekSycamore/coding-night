import Post from "./Post";

const PostsDisplay = () => {
    return(
        <section className="bg-white dark:bg-gray-900 w-full">
            <div className="mx-auto max-w-lg">
        <h1 className="text-3xl font-bold text-gray-800 dark:text-white lg:text-4xl">You are looking for a place to stay? Find here volunteers to help</h1>
        <p className="mt-3 text-sm text-gray-400 text-center">No payment required</p>
        </div>
        <div className="flex flex-col w-full space-y-5 mt-10">
                <span className="w-full border border-secondary"></span></div>
             <div className="min-h-screen bg-gradient-to-tr from-red-300 to-yellow-200 flex justify-center items-center py-20">
            <div className="md:px-4 md:grid md:grid-cols-2 lg:grid-cols-3 gap-5 space-y-4 md:space-y-0">
        <Post username="Cieciw"   title="przenocuje"
    content="rudzy jest porabany dyuasgyufyusgfuisgyufausigfyuasguyfgasuifgyusgfuiysfuigsuifguaygfuiawguiawgf"
    location="Warszawa"/>
     <Post username="Cieciw"   title="przenocuje"
    content="rudzy jest porabany dyuasgyufyusgfuisgyufausigfyuasguyfgasuifgyusgfuiysfuigsuifguaygfuiawguiawgf"
    location="Warszawa"/>
     <Post username="Cieciw"   title="przenocuje"
    content="rudzy jest porabany dyuasgyufyusgfuisgyufausigfyuasguyfgasuifgyusgfuiysfuigsuifguaygfuiawguiawgf"
    location="Warszawa"/>
     <Post username="Cieciw"   title="przenocuje"
    content="rudzy jest porabany dyuasgyufyusgfuisgyufausigfyuasguyfgasuifgyusgfuiysfuigsuifguaygfuiawguiawgf"
    location="Warszawa"/>
    </div>
    </div>
    </section>
    )
}

export default PostsDisplay;