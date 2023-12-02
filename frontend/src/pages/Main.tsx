import Navbar from './components/Navbar';
import PostsDisplay from './components/PostsDisplay';

const Main = () => {
    return(
        <>
        <Navbar/>
<script src="//unpkg.com/alpinejs" defer></script>

<main className="bg-sky-500">

    <PostsDisplay/>

</main>
        </>
    );
}
export default Main;