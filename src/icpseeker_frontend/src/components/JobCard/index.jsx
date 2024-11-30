import * as React from "react";

const JobCard = ({ name, role, image, description }) =>  {
  return (
    <div className="flex overflow-hidden flex-col p-5 bg-white rounded-lg border border-solid border-zinc-300 min-w-[240px] w-[356px]">
      <div className="flex gap-5 items-center">
        <div className="flex gap-5 items-center self-stretch my-auto text-violet-900 min-w-[240px]">
          <img
            loading="lazy"
            src={image}
            alt={`${name}'s profile picture`}
            className="object-contain shrink-0 self-stretch my-auto w-20 rounded-2xl aspect-square"
          />
          <div className="flex flex-col self-stretch my-auto w-[168px]">
            <div className="flex flex-col justify-center items-start w-full text-xl font-bold leading-tight min-h-[32px]">
              <div className="gap-2 self-stretch text-ellipsis w-[167px]">
                {name}
              </div>
            </div>
            <div className="gap-1 self-stretch px-4 py-2 mt-1 text-sm font-semibold border-2 border-violet-900 border-solid rounded-[100px]">
              {role}
            </div>
          </div>
        </div>
        <button 
          className="flex overflow-hidden gap-2.5 items-center self-stretch px-2.5 py-1.5 my-auto w-6"
          aria-label={`More options for ${name}`}
        >
          <img
            loading="lazy"
            src="https://cdn.builder.io/api/v1/image/assets/TEMP/617a664a1a47983e9ebdebdd8ea436144ec948145210386145d71e4add9d912c?placeholderIfAbsent=true&apiKey=cb3993b2e7204330b96b48ee5ae502db"
            alt=""
            className="object-contain self-stretch my-auto w-1.5 aspect-[0.5] stroke-[2px] stroke-violet-900"
          />
        </button>
      </div>
      <div className="mt-6 w-full text-base leading-6 text-violet-700 whitespace-nowrap text-ellipsis">
        {description}
      </div>
    </div>
  );
}

export default JobCard;
