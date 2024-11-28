import React from "react";
import { NavigationItems } from "./NavigationItems.jsx";
import AuthButton from "../Auth/AuthButton"; // Pastikan path sudah benar
import Logo from "../../assets/logo.svg"; 

export default function Navbar() {
    return (
        <header className="flex justify-between items-center w-full px-12 py-3 bg-gradient-to-r from-gradient-start to-gradient-end shadow-md">
            <div className="flex-initial">
                <img
                    src={Logo}
                    alt="ICP Seeker Logo"
                    className="h-10 w-auto" // Customize size as needed
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
