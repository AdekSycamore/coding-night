import {useRef, ChangeEvent} from 'react';

const SignUp = () => {

  const nameRef = useRef<string>('');
  const surnameRef = useRef<string>('');
  const typeRef = useRef<string>('');
  const userRef = useRef<string>('');
  const passRef = useRef<string>('');
  const secpassRef = useRef<string>('');

  const NameChange = (event: ChangeEvent<HTMLInputElement>):void => {
    nameRef.current = event.target.value;
  }
  const SurnameChange = (event: ChangeEvent<HTMLInputElement>):void => {
    surnameRef.current = event.target.value;
  }
  const TypeChange = (event: ChangeEvent<HTMLSelectElement>):void => {
    typeRef.current = event.target.value;
  }

  const UserChange = (event: ChangeEvent<HTMLInputElement>):void => {
    userRef.current = event.target.value;
  }

  const PassChange = (event: ChangeEvent<HTMLInputElement>):void => {
    passRef.current = event.target.value;
  }

  const SecPassChange = (event: ChangeEvent<HTMLInputElement>):void => {
    secpassRef.current = event.target.value;
  }

  const VerifyPassword = () => {
    const dec: HTMLSpanElement | null = (document.getElementById("dec") as HTMLSpanElement);
    if(passRef != secpassRef){
      dec.textContent = "Passwords do not matches!";
    }
  }

    return(
<div className="bg-white">

  <div className="flex min-h-screen">

    <div className="flex flex-row w-full">

      <div className="flex flex-1 flex-col items-center justify-center px-10 relative">
            <div className="flex flex-1 flex-col  justify-center space-y-5 max-w-md">
          <div className="flex flex-col space-y-2 text-center">
            <h2 className="text-3xl md:text-4xl font-bold">Create an account</h2>
          </div>
          <div className="flex flex-col max-w-md space-y-5">
            <input type="text" placeholder="Name" onChange={NameChange}
              className="flex px-3 py-2 md:px-4 md:py-3 border-2 border-black rounded-lg font-medium placeholder:font-normal" />
          </div>
          <div className="flex flex-col max-w-md space-y-5">
            <input type="text" placeholder="Surname" onChange={SurnameChange}
              className="flex px-3 py-2 md:px-4 md:py-3 border-2 border-black rounded-lg font-medium placeholder:font-normal" />
          </div>
          <div className="flex flex-col max-w-md space-y-5">
            <select onChange={TypeChange}
              className="flex px-3 py-2 md:px-4 md:py-3 border-2 border-black rounded-lg font-medium placeholder:font-normal text-[#c2c2c2]">
                <option value="Volunteer">The Volunteer</option>
                <option value="Needy">The Needy</option>
                <option value="Both">In Between</option>
              </select>
          </div>
          <div className="flex flex-col max-w-md space-y-5">
            <input type="text" placeholder="Username" onChange={UserChange}
              className="flex px-3 py-2 md:px-4 md:py-3 border-2 border-black rounded-lg font-medium placeholder:font-normal" />
          </div>
          <div className="flex flex-col max-w-md space-y-5">
            <input type="password" placeholder="Password" onChange={PassChange}
              className="flex px-3 py-2 md:px-4 md:py-3 border-2 border-black rounded-lg font-medium placeholder:font-normal" />

            <input type="password" placeholder="Verify password" onChange={SecPassChange}
              className="flex px-3 py-2 md:px-4 md:py-3 border-2 border-black rounded-lg font-medium placeholder:font-normal" />

              <span className="w-full border border-black"></span>
              
            <button onClick={VerifyPassword} className="flex items-center justify-center flex-none px-3 py-2 md:px-4 md:py-3 border-2 rounded-lg font-medium border-black bg-black text-white">Confirm</button>
            <span id="dec" className="flex flex-col items-center w-full text-sm text-[#f00]"></span>
          </div>
          </div>
        </div>
        <div className="hidden lg:flex flex-col justify-between bg-[url('./assets/charity.jpg')] lg:p-8 xl:p-12 lg:max-w-sm xl:max-w-lg">

<div>
  <h1 className="lg:text-3xl xl:text-5xl xl:leading-snug font-extrabold mt-6 text-white">Be the blessing that others need!</h1>
</div>
</div>
        </div>

      </div>

  </div>
    )
}

export default SignUp;