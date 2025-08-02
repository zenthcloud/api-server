/**
 * ==============================================================
 * Project     : Zenth Cloud – Zenth Panel
 * File        : index.ts
 * Version     : 1.0.0
 * Description : API Entry Point
 * Author      : Sky Genesis Enterprise
 * Created on  : 2025-07-19
 * License     : AGPLv3
 * Forked from : N/A
 * Modified by : Liam Dispa <liam.dispa@skygenesisenterprise.com> (2025-08-02)
 * ==============================================================
 */

import express, { Request, Response, NextFunction } from 'express';
import { createServer } from 'http';
import { Server as SocketIOServer } from 'socket.io';
import cors from 'cors';
import dotenv from 'dotenv';

// Variables d'environnement (tu peux utiliser dotenv)
const PORT = process.env.PORT || 4000;
const VALID_API_KEYS = new Set([
  'clé-api-1', // à remplacer ou charger dynamiquement
  'clé-api-2',
]);

// Middleware pour vérifier la clé API dans header 'x-api-key'
function apiKeyMiddleware(req: Request, res: Response, next: NextFunction) {
  const apiKey = req.header('x-api-key');
  if (!apiKey || !VALID_API_KEYS.has(apiKey)) {
    return res.status(401).json({ error: 'Unauthorized: Invalid API Key' });
  }
  next();
}

// Création de l'app Express
const app = express();

// Middleware global
app.use(cors());
app.use(express.json()); // pour parser JSON

// Route publique non protégée
app.get('/health', (_req, res) => {
  res.json({ status: 'ok', time: new Date().toISOString() });
});

// Routes protégées par clé API
app.use('/api', apiKeyMiddleware);

// Exemple route protégée
app.get('/api/user-info', (req: Request, res: Response) => {
  // Simule une réponse utilisateur
  res.json({
    user: 'client123',
    permissions: ['read', 'write'],
    roles: ['user'],
  });
});

// Gestion d’erreurs basique
app.use((err: Error, _req: Request, res: Response, _next: NextFunction) => {
  console.error(err.stack);
  res.status(500).json({ error: 'Internal Server Error' });
});

// Création serveur HTTP & Socket.IO
const httpServer = createServer(app);
const io = new SocketIOServer(httpServer, {
  cors: {
    origin: '*', // restreindre en prod !
    methods: ['GET', 'POST'],
  },
});

// WebSocket basique
io.on('connection', (socket) => {
  console.log('Nouvelle connexion WebSocket:', socket.id);

  socket.on('ping', () => {
    socket.emit('pong');
  });

  socket.on('disconnect', () => {
    console.log('Client déconnecté:', socket.id);
  });
});

// Démarrer le serveur
httpServer.listen(PORT, () => {
  console.log(`API Zenth Cloud démarrée sur le port ${PORT}`);
});