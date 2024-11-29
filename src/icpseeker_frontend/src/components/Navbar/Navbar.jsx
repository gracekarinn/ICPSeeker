import React from "react";
import { NavigationItems } from "./NavigationItems.jsx";
import AuthButton from "../Auth/AuthButton";
import Logo from "../../assets/logo.svg"; 

export default function Navbar() {
    return (
        <header className="sticky top-0 z-50 flex justify-between items-center w-full px-12 py-3 bg-gradient-to-r from-gradient-start to-gradient-end shadow-md border-b-4 border-purple-light">
            <div className="flex-initial">
                <img
                    src={Logo}
                    alt="ICP Seeker Logo"
                    className="h-10 w-auto" 
                />
            </div>
            <nav className="flex-grow flex justify-center text-white text-lg">
                <NavigationItems />
            </nav>
            <div className="flex-initial">
                <AuthButton />  
            </div>
        </header>
    );
}
