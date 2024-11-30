import React, { useState } from 'react';
import { Link } from 'react-router-dom';
import Logo from "../../assets/logo.svg";
import Button from "../../components/Button";
import Icon from "../../assets/Scroll.png";

const Sidebar = () => {
  const [activeItem, setActiveItem] = useState('jobPostings'); 

  return (
    <div className="flex flex-col w-[18%] max-md:ml-0 max-md:w-full">
      <div className="flex relative flex-col w-full">
        <div className="flex z-0 gap-2 items-center px-5 py-6 w-full text-white bg-purple-dark max-w-[257px] min-h-[100px]">
          <div className="flex flex-1 shrink gap-9 items-center self-stretch my-auto w-full basis-0">
            <div className="flex flex-col flex-1 shrink self-stretch my-auto basis-0">
              <div className="text-sm font-bold">Test</div>
              <Button className="bg-gradient-to-r from-purple-normal to-blue-normal hover:from-purple-normal-hover hover:to-blue-normal-hover" size="small">
                View Profile
              </Button>
            </div>
            <img
              src={Logo}
              alt={``}
              className="object-contain shrink-0 self-stretch my-auto rounded-full aspect-square shadow-[0px_3px_3px_rgba(0,0,0,0.25)] w-[52px]"
            />
          </div>
        </div>
        <nav className="flex z-0 flex-col flex-1 justify-between px-5 pt-9 pb-3 w-full bg-violet-700 max-w-[257px]">
          <div className="flex flex-col w-full text-base font-bold leading-tight text-white">
            <div className="flex flex-col w-full">
              {['jobPostings', 'projects', 'employees', 'payroll'].map(item => (
                <Link to={`/jobpostings`} key={item}>
                  <button
                    className={`flex gap-4 items-center px-3 py-4 w-full ${
                      activeItem === item ? 'text-violet-700 bg-violet-100' : 'text-white'
                    } rounded-2xl min-h-[54px]`}
                    onClick={() => setActiveItem(item)}
                  >
                    <img
                      src={Icon} // Ensure you have these images in your public/assets or src/assets directory
                      alt={item}
                      className="object-contain shrink-0 self-stretch my-auto w-6 aspect-square"
                    />
                    <div className="self-stretch my-auto w-[143px]">
                      <h6 className="text-h6">{item.charAt(0).toUpperCase() + item.slice(1)}</h6>
                    </div>
                  </button>
                </Link>
              ))}
            </div>
          </div>
          <button className="gap-2 self-stretch px-5 py-1.5 w-full text-xs font-semibold text-violet-900 rounded-xl mt-[626px] max-md:mt-10">
            Log Out
          </button>
        </nav>
      </div>
    </div>
  );
};

export default Sidebar;
