## PACKAGE-PIPELINE

Il s'agit d'un service qui écoute lorsqu'une nouvelle version d'un package est disponible sur Github Containers,\
et envoie une requête de redémarrage à un service particulier dans le réseau kube associé.

### *Cas d'usage*
Je me sers de Github Containers pour stocker les images de mes services.\
Et j'hébèrge mes services dans un réseau kube. 

**PACKAGE-PIPELINE** permet d'automatiser mon "workflow" de sorte à ce que \
dès que j'envoie l'image de mon service, \
il puisse directement mis en production.

### Comment

1. Avoir un cluster kube opérationnel
2. Démarrer le service avec la variable d'environnement `GITHUB_WEBHOOK_SECRET`\
Il écoutera sur le port 80.
3. Ajouter comme label à vos déploiements `nogata.package-pipeline: {nom de l'image}`
4. Enregistrer les webhooks sur vos repositories en cochant uniquement le webhook "Packages" et en donnant le bon secret (`GITHUB_WEBHOOK_SECRET`)

*nom de l'image de `ghcr.io/loshido/neon-dawn` -> `neon-dawn`

### Erreurs/Problèmes connues

- l'étiquette `nogata.package-pipeline` devrait être changé en `package-pipeline` si l'usage de ce service dépasse mon usage personnel.
- Il faudrait permettre plus d'actions en réponse aux webhooks.
- Il faudrait mieux expliquer ce que fait ce projet.