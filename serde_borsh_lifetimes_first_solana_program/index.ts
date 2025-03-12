import express from 'express';

const app = express();

app.use(function(err,req,res,next){
    if(err){
        res.json({
            message:err.message
        })
    }
})