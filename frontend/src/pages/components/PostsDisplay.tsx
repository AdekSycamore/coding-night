import Post from "./Post";

const PostsDisplay = () => {
    return(
        <section className="bg-white dark:bg-gray-900 w-full">
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