# 🧪 Technology Environment – HASH HUNTER

HASH HUNTER est une **plateforme communautaire de CTF orientée blockchain**, permettant la création, la résolution et le partage de challenges techniques sur différentes blockchains. Ce document décrit les choix techniques majeurs qui soutiennent le développement de la plateforme.

---

## ⚙️ Technologies utilisées

### Front-end

- **React** – pour la construction d’une interface utilisateur interactive et dynamique.
- **TypeScript** – pour renforcer la robustesse du code front-end avec un typage statique.

### Back-end

- **Rust** – langage performant et sécurisé, utilisé pour construire des services back-end fiables.
- **Axum** – framework web basé sur Tokio, adapté pour créer des API performantes en Rust.
- **PostgreSQL** – base de données relationnelle robuste pour stocker les données utilisateurs, challenges, résultats, etc.
- **Kubernetes** – orchestrateur de conteneurs pour le déploiement, la scalabilité et la gestion de l'infrastructure de manière automatisée.

---

## 🧱 Architecture

L’architecture de HASH HUNTER suit un modèle **modulaire et scalable**, facilitant :

- L'ajout de nouvelles blockchains.
- La montée en charge progressive via Kubernetes.
- La séparation claire entre les différentes couches (interface utilisateur, logique métier, stockage des données).

---

## 📦 Déploiement & Scalabilité

Grâce à **Kubernetes**, l’environnement de production est :

- Hautement disponible.
- Facile à maintenir.
- Capable de s’adapter à une augmentation du nombre d’utilisateurs ou de challenges.

Utilisation d'un serveur O2Switch avec l'offre Cloud (16€/mois)

---

## 💡 Conclusion

HASH HUNTER repose sur un écosystème technologique moderne et performant, conçu pour offrir une expérience fluide aux utilisateurs tout en garantissant robustesse et sécurité du système.
