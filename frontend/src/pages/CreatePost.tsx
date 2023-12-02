import {useRef, ChangeEvent} from "react";
import Navbar from "./components/Navbar";
const CreatePost = () => {
    const userRef = useRef<string>('');
    const titleRef = useRef<string>('');
    const locRef = useRef<string>('');
    const imgRef = useRef<string>('');
    const contentRef = useRef<string>('');

    const UserChange = (event: ChangeEvent<HTMLInputElement>): void => {
        userRef.current = event.target.value;
    }
    const TitleChange = (event: ChangeEvent<HTMLInputElement>): void => {
        titleRef.current = event.target.value;
    }
    const LocChange = (event: ChangeEvent<HTMLInputElement>): void => {
        locRef.current = event.target.value;
    }
    const ImgChange = (event: ChangeEvent<HTMLInputElement>): void => {
        imgRef.current = event.target.value;
    }

    const ContentChange = (event: ChangeEvent<HTMLTextAreaElement>): void => {
        contentRef.current = event.target.value;
    }

    return(
        <div>
        <Navbar/>
        <div className="flex flex-row h-full w-full bg-[url('./pages/assets/city.jpg')] bg-cover object-fill ">
        <div className="w-2/3 mx-auto my-8 items-center justify-center h-window bg-[#fff] border border-[#000] rounded p-12">
        <form className="w-full h-full items-center justify-center mx-auto">

  <div className="flex flex-wrap mx-3 mb-6">
    <div className="w-full md:w-1/2 px-3 mb-6 md:mb-0">
      <label className="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2">
        Username
      </label>
      <input onChange={UserChange} className="appearance-none block w-full bg-gray-200 text-gray-700 border border-red-500 rounded py-3 px-4 mb-3 leading-tight focus:outline-none focus:bg-white" id="grid-first-name" type="text" placeholder="Username"></input>
    </div>
    <div className="w-full md:w-1/2 px-3">
      <label className="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2">
        Title
      </label>
      <input onChange={TitleChange} className="appearance-none block w-full bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 px-4 leading-tight focus:outline-none focus:bg-white focus:border-gray-500" id="grid-last-name" type="text" placeholder="Title"></input>
    </div>
  </div>
  <div className="flex flex-wrap mx-3 mb-6">
  <div className="w-full md:w-1/2 px-3 mb-6 md:mb-0">
      <label className="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2">
        Location
      </label>
      <input onChange={LocChange} className="appearance-none block w-full bg-gray-200 text-gray-700 border border-red-500 rounded py-3 px-4 mb-3 leading-tight focus:outline-none focus:bg-white" id="grid-first-name" type="text" placeholder="Location"></input>
    </div>
    <div className="w-full md:w-1/2 px-3">
      <label className="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2">
        Img Source
      </label>
      <input onChange={ImgChange} className="appearance-none block w-full bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 px-4 leading-tight focus:outline-none focus:bg-white focus:border-gray-500" id="grid-last-name" type="text" placeholder="Img source"></input>
    </div>
  </div>
  <div className="flex flex-wrap mx-3 mb-6">
    <div className="w-full px-3">
      <label className="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2" >
        Content
      </label>
      <textarea onChange={ContentChange} className=" no-resize appearance-none block w-full bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 px-4 mb-3 leading-tight focus:outline-none focus:bg-white focus:border-gray-500 h-48 resize-none" id="message"></textarea>
    </div>
  </div>
  <div className="flex items-center w-full mx-3">
    <div className="w-full justify-center items-center ">
      <button className="w-full bg-[#000] text-[#fff] font-bold py-2 px-4 rounded" type="button">
        Send
      </button>
    </div>
  </div>
</form>
</div>
</div>
</div>
    )
}

export default CreatePost;