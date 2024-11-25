import React from "react";
import { NavigationItems } from "./NavigationItems.jsx";
import Button from "./button/index.jsx";

export function Navbar() {
    return (
        <header
            className="flex flex-wrap gap-10 justify-between items-center px-12 py-3 border-b-4 border-solid border-b-violet-100 max-md:px-5">
            <img
                loading="lazy"
                src="https://s3.coinmarketcap.com/static-gravity/image/2fb1bc84c1494178beef0822179d137d.png"
                alt="Company Logo"
                className="object-contain shrink-0 self-stretch my-auto aspect-[3.18] w-[191px]"
            />
            <nav
                className="flex flex-wrap gap-8 items-center self-stretch my-auto text-base font-bold leading-tight text-white min-w-[240px] max-md:max-w-full">
                <NavigationItems/>
            </nav>
            <Button variant="secondary" size="medium">
                Log in
            </Button>
        </header>
    )
}
