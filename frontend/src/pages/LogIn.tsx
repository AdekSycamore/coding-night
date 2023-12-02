import { useRef, ChangeEvent } from 'react';
import { Link } from "react-router-dom";
import Navbar from './components/Navbar';


const LogIn = () => {

  const userRef = useRef<string>('');
  const passRef = useRef<string>('');

  const InputChange = (event: ChangeEvent<HTMLInputElement>): void => {
    userRef.current = event.target.value;
  }

  const PassChange = (event: ChangeEvent<HTMLInputElement>): void => {
    passRef.current = event.target.value;
  }


  return (
    <div className="bg-white">
      <div className="flex min-h-screen">
        <div className="flex flex-row w-full">

          <div className="hidden lg:flex flex-col justify-between bg-[url('./pages/assets/help.jpg')] lg:p-8 xl:p-12 lg:max-w-sm xl:max-w-lg" >

            <div className="flex flex-row w-full">

              <div className="hidden lg:flex flex-col justify-between bg-[url('./pages/assets/help.jpg')] lg:p-8 xl:p-12 lg:max-w-sm xl:max-w-lg">

                <div>
                  <h1 className="lg:text-3xl xl:text-5xl xl:leading-snug font-extrabold mt-6 text-[#fff]">Helping others is the secret to a happy life!</h1>
                </div>
              </div>
              </div>
            </div>

                  <div className="flex flex-1 flex-col px-10 relative h-screen">
                    <Navbar/>
                    <div className="flex items-center justify-center h-screen relative">
                    
                    <div className="flex flex-1 flex-col  justify-center space-y-7 max-w-md">
                      <div className="flex flex-col space-y-2 text-center">
                        <h2 className="text-3xl md:text-4xl font-bold">Sign in to account</h2>
                      </div>
                      <div className="flex flex-col max-w-md space-y-5">
                        <input type="text" placeholder="Username" onChange={InputChange}
                          className="flex px-3 py-2 md:px-4 md:py-3 border-2 border-black rounded-lg font-medium placeholder:font-normal" />
                      </div>
                      <div className="flex flex-col max-w-md space-y-5">
                        <input type="password" placeholder="Password" onChange={PassChange}
                          className="flex px-3 py-2 md:px-4 md:py-3 border-2 border-black rounded-lg font-medium placeholder:font-normal" />
                        <span className="w-full border border-black"></span>
                        <button className="flex items-center justify-center flex-none px-3 py-2 md:px-4 md:py-3 border-2 rounded-lg font-medium border-[#000] bg-[#000] text-[#fff]">Confirm</button>
                        <Link to="/signup"><span className="flex flex-col items-center w-full text-sm cursor-pointer">Create an account here!</span></Link>
                      </div>
                    </div>
                    </div>
                  </div>
                  </div>
                </div>
      </div>
  )
}

export default LogIn;