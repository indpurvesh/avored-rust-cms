import{d as m,u as c,r as n,c as f,a as t,w as p,b as i,v as r,_ as x,e as w,o as _,f as h}from"./index-0a1a9a14.js";const g={class:"bg-white h-screen flex items-center max-w-lg mx-auto"},b={class:"flex items-center justify-center flex-col mx-auto shadow py-12 p-5 w-full"},v=t("div",{class:"flex w-full justify-center"},[t("a",{class:"",href:"/"},[t("img",{src:x,class:"w-16"})]),t("h1",{class:"mt-3 text-xl font-bold text-red-700"}," AvoRed Rust CMS ")],-1),y={class:"flex w-full"},k=["onSubmit"],S={class:"w-full"},R=t("label",{for:"Email",class:"w-full text-sm font-medium text-gray-700"}," Email ",-1),E={class:"mt-5"},V=t("label",{for:"Password",class:"block text-sm font-medium text-gray-700"}," Password ",-1),j=t("div",{class:"mt-5 flex w-full items-center"},[t("button",{type:"submit",class:"shrink-0 rounded-md border border-blue-600 bg-blue-600 px-8 py-3 text-sm font-medium text-white transition hover:bg-transparent hover:text-blue-600 focus:outline-none focus:ring active:text-blue-500"}," Login "),t("p",{class:"ml-auto text-sm text-gray-500"},[t("a",{href:"#",class:"text-gray-700 underline"},"forgot your password?"),w(". ")])],-1),M=m({__name:"Login",setup(B){const d=c(),s=n("admin@admin.com"),o=n("admin123"),u=async()=>{const l={email:s.value,password:o.value},e=await h.post("/api/auth/login",JSON.stringify(l),{headers:{"Content-Type":"application/json"}});e.data.token&&(localStorage.setItem("is_logged_in","true"),localStorage.setItem("token",e.data.token),d.push({path:"/"}))};return(l,e)=>(_(),f("section",g,[t("div",b,[v,t("div",y,[t("form",{onSubmit:p(u,["prevent"]),action:"#",class:"mt-3 w-full"},[t("div",S,[R,i(t("input",{type:"email","onUpdate:modelValue":e[0]||(e[0]=a=>s.value=a),autofocus:"",id:"Email",name:"email",class:"mt-1 block w-full rounded-md border-gray-200 bg-white text-sm text-gray-700 shadow-sm"},null,512),[[r,s.value]])]),t("div",E,[V,i(t("input",{type:"password",id:"Password","onUpdate:modelValue":e[1]||(e[1]=a=>o.value=a),name:"password",class:"mt-1 w-full rounded-md border-gray-200 bg-white text-sm text-gray-700 shadow-sm"},null,512),[[r,o.value]])]),j],40,k)])])]))}});export{M as default};