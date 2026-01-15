// Translations
const translations = {
    en: {
        'nav.about': 'About',
        'nav.services': 'Services',
        'nav.paradise': 'Paradise',
        'nav.projects': 'Projects',
        'nav.contact': 'Contact',
        'hero.tagline': 'Friendly Honest Advice Since 2006',
        'hero.subtitle': 'Now serving from Puerto Morelos, Mexico',
        'hero.services': 'Our Services',
        'hero.contact': 'Get in Touch',
        'about.title': 'Our Story',
        'about.heading': 'From St. Louis Storefront to Caribbean Consulting',
        'about.p1': 'South City Computer started in 2006 on Ivanhoe Avenue in the Lindenwood Park neighborhood of St. Louis. For years, we provided computer repair, sales, and honest technical advice to the local community from our signature storefront with its colorful window signs and industrial-chic interior.',
        'about.p2': 'Today, we\'ve traded the Gateway to the West for the Caribbean coast. Based in Puerto Morelos, Mexico, we\'ve evolved from hardware repair to software craftsmanship. Our focus is now on consulting, custom application development, and systems programming—bringing the same friendly, honest approach that built our reputation.',
        'about.p3': 'Whether you need a Rust CLI tool, a Django web application, or expert guidance on your software architecture, we bring decades of hands-on experience from the trenches of technology.',
        'about.cap1': 'The original St. Louis storefront',
        'about.cap2': 'Our signature style',
        'about.cap3': 'Always hands-on',
        'about.cap4': 'Local art and culture',
        'banner.beach.title': 'Now Based in Paradise',
        'banner.beach.subtitle': 'Puerto Morelos, Mexico',
        'banner.beach.cta': 'Meet Me in Paradise',
        'banner.malecon.title': 'Caribbean Craftsmanship',
        'banner.malecon.subtitle': 'Quality code from the Riviera Maya',
        'banner.harbor.title': 'Let\'s Build Something',
        'banner.harbor.subtitle': 'Your next project starts here',
        'services.title': 'Software Development Services',
        'services.intro': 'Two decades of experience solving real problems. Professional software development with transparent pricing and honest timelines. We handle the technical details so you can focus on your business.',
        'services.consulting.title': 'Software Consulting',
        'services.consulting.desc': 'Architecture reviews, code audits, and technical guidance. We help you make the right decisions before you write a single line of code.',
        'services.dev.title': 'Custom App and Web Development',
        'services.dev.desc': 'From CLI tools in Rust to full-stack web applications with Django. We build exactly what you need, nothing more, nothing less.',
        'services.systems.title': 'Systems Programming',
        'services.systems.desc': 'Unix/Linux infrastructure, tooling, automation, and performance optimization. The unglamorous work that makes everything else possible.',
        'services.ai.title': 'AI/ML Integration',
        'services.ai.desc': 'Practical applications of language models and data analysis. Not hype—real solutions that add value to your workflows.',
        'services.it.heading': 'Local IT Support',
        'services.it.intro': 'We selectively perform repairs and on-site IT services for local clients. We are no longer a walk-in brick and mortar business. Contact us to see if your problem is one we handle.',
        'services.it.repair': 'Computer Repair',
        'services.it.recovery': 'Data Recovery',
        'services.it.business': 'Business IT',
        'services.it.support': 'Computer Support',
        'services.it.sales': 'Computer Sales',
        'services.it.recycle': 'Computer Recycling',
        'projects.title': 'Projects',
        'projects.intro': 'Open source tools and applications we\'ve built. Quality code you can inspect.',
        'contact.title': 'Get in Touch',
        'contact.intro': 'Have a project in mind? Let\'s talk about how we can help.',
        'contact.name': 'Name *',
        'contact.email': 'Email *',
        'contact.phone': 'Phone',
        'contact.message': 'Message *',
        'contact.send': 'Send Message',
        'contact.sending': 'Sending...',
        'contact.chat': 'Live Chat',
        'contact.coming': 'Coming Soon',
        'contact.indev': 'In Development',
        'contact.location': 'Location',
        'contact.address': 'Puerto Morelos, Riviera Maya<br>Between Cancun & Playa Del Carmen<br>Mexico',
        'contact.facebook': 'Message us on Facebook',
        'contact.helpdesk.title': 'Support Helpdesk',
        'contact.helpdesk.desc': 'Check your support ticket status',
        'contact.helpdesk.login': 'Login to Helpdesk',
        'footer.tagline': 'Friendly Honest Advice Since 2006',
        'footer.copyright': '© 2006–2026 South City Computer. All rights reserved.\nBuilt in Rust at South City Computer.',
        'footer.rating': 'out of 200+ reviews',
        'footer.connect': 'Connect',
        'footer.download': 'Download our App',
        'projects.case.label': 'Case Study',
        'projects.case.title': 'The 52ms Website',
        'projects.case.desc': 'How we built this site to load 46x faster than average—using Rust and WebP. Read why speed matters for your business.',
        'projects.web.label': 'Web Design',
        'projects.web.title': 'Pet Friendly Vet',
        'projects.web.desc': 'Professional veterinary website generated from Facebook business content using AI. Fast, mobile-friendly, and SEO-optimized.',
        'projects.viewall.title': 'View All Projects',
        'projects.viewall.desc': 'See our complete portfolio of client work and open source tools.',
        'paradise.title': 'Meet Me in Paradise',
        'paradise.tagline': 'Schedule a vacation with the money you\'ll save hiring us. Plan your project surrounded by natural beauty.',
        'paradise.intro': 'Like travel dentistry, but for tech. Fly to the Riviera Maya, meet face-to-face, and leave with a working prototype—plus a tan. I\'m more relaxed and productive here, and that energy transfers directly into better work for you.',
        'paradise.packages': 'Retreat Packages',
        'paradise.tech.title': 'Tech Retreat',
        'paradise.tech.desc': 'Rapid prototyping sessions, architecture planning, hands-on development. Leave with working software and deployment documentation.',
        'paradise.mentor.title': 'Business Mentorship',
        'paradise.mentor.desc': 'Strategic coaching sessions with experienced entrepreneurs. Business model development, market positioning, growth strategy.',
        'paradise.story.title': 'Documentary Storytelling',
        'paradise.story.desc': 'Learn to tell your brand\'s story through video. Coaching on narrative structure, visual storytelling, and authentic content creation.',
        'paradise.wellness.title': 'Wellness & Adventure',
        'paradise.wellness.desc': 'Morning yoga, cenote tours, snorkeling the reef, jungle exploration. Recharge while you strategize—the best ideas come when you\'re relaxed.',
        'paradise.cta': 'Combine consulting with Caribbean adventure. Remote work welcome. On-site retreats available.',
        'paradise.button': 'Plan Your Retreat',
        'review.1': '"South City went above and beyond. I work online and rely on my laptop daily. Nestor fixed all issues on a rush basis and I didn\'t lose any working hours. He\'s extremely knowledgeable and patient with luddites, too. If I could give more than 5 stars, I would!"',
        'review.2': '"They are the best! I have never worked with a better computer tech company. This company diagnosed my issue and fixed it. It was complicated but he solved my problem. I am a small business. Nestor is officially our new IT department."',
        'review.3': '"The staff here is super helpful, thoughtful and great to work with! The owner does a lot of the repairs himself and does quality work! He was able to get the water damaged laptop up and running again!"',
        'review.4': '"These guys always find the way to get you going. Our computer was completely disabled, still they saved the important info and even borrowed an old computer to complete the presentation that was due that day. They saved the day!"',
        'review.5': '"Great service, quick, complete. Katrina is a saint as she walked me through the process to make transferred files accessible. Recommend to anyone needing top notch, professional computer service."',
        'review.6': '"Absolutely amazing experience! I utterly panicked when my laptop would no longer turn on, right in the middle of finals week, and I didn\'t know what I was going to do. They had my laptop fixed and ready in less than 24 hours!"',
        'review.7': '"These guys really know what they\'re doing! They are extremely knowledgeable and have a delightfully quirky space that is fun to visit. Their support ticket system is easy to use and keeps you updated."',
        'review.8': '"Took in my wife\'s computer that was running really slow and freezing up. South City tested the computer, backed up all the files, installed a new hard drive and re-installed the files quickly and at a great price."',
        'review.9': '"Great service...reasonable prices...they took my old laptop and rebuilt the unit into a solid state..installed Linux..installed some new programs... best place to go!"',
        'review.10': '"South City Computer site had an easy to use ticket system, he got back to me quickly, was flexible in scheduling and definitely showed concern for transparency as he walked me through each step."',
        'review.11': '"My daughter damaged the screen to her laptop. They repaired it like it was brand new. Once the part arrived the repair was made that day. The charge for service seemed very reasonable."',
        'review.12': '"Excellent work. Have used them before, even buying a refurbished computer from them several years ago. The only place I would consider taking my computer. Would not hesitate to recommend them."'
    },
    es: {
        'nav.about': 'Nosotros',
        'nav.services': 'Servicios',
        'nav.paradise': 'Paraíso',
        'nav.projects': 'Proyectos',
        'nav.contact': 'Contacto',
        'hero.tagline': 'Consejos Honestos y Amigables Desde 2006',
        'hero.subtitle': 'Ahora sirviendo desde Puerto Morelos, México',
        'hero.services': 'Nuestros Servicios',
        'hero.contact': 'Contáctanos',
        'about.title': 'Nuestra Historia',
        'about.heading': 'De Tienda en St. Louis a Consultoría en el Caribe',
        'about.p1': 'South City Computer comenzó en 2006 en Ivanhoe Avenue en el vecindario de Lindenwood Park en St. Louis. Durante años, brindamos reparación de computadoras, ventas y asesoría técnica honesta a la comunidad local desde nuestra distintiva tienda con sus coloridos letreros y su interior industrial-chic.',
        'about.p2': 'Hoy, hemos cambiado la Puerta del Oeste por la costa del Caribe. Ubicados en Puerto Morelos, México, hemos evolucionado de la reparación de hardware a la artesanía del software. Nuestro enfoque ahora está en consultoría, desarrollo de aplicaciones personalizadas y programación de sistemas—manteniendo el mismo enfoque amigable y honesto que construyó nuestra reputación.',
        'about.p3': 'Ya sea que necesites una herramienta CLI en Rust, una aplicación web con Django, o guía experta en tu arquitectura de software, traemos décadas de experiencia práctica desde las trincheras de la tecnología.',
        'about.cap1': 'La tienda original en St. Louis',
        'about.cap2': 'Nuestro estilo distintivo',
        'about.cap3': 'Siempre prácticos',
        'about.cap4': 'Arte y cultura local',
        'banner.beach.title': 'Ahora en el Paraíso',
        'banner.beach.subtitle': 'Puerto Morelos, México',
        'banner.beach.cta': 'Encuéntrame en el Paraíso',
        'banner.malecon.title': 'Artesanía Caribeña',
        'banner.malecon.subtitle': 'Código de calidad desde la Riviera Maya',
        'banner.harbor.title': 'Construyamos Algo',
        'banner.harbor.subtitle': 'Tu próximo proyecto empieza aquí',
        'services.title': 'Servicios de Desarrollo de Software',
        'services.intro': 'Dos décadas de experiencia resolviendo problemas reales. Desarrollo de software profesional con precios transparentes y plazos honestos. Nos encargamos de los detalles técnicos para que puedas enfocarte en tu negocio.',
        'services.consulting.title': 'Consultoría de Software',
        'services.consulting.desc': 'Revisiones de arquitectura, auditorías de código y guía técnica. Te ayudamos a tomar las decisiones correctas antes de escribir una sola línea de código.',
        'services.dev.title': 'Desarrollo de Apps y Web a Medida',
        'services.dev.desc': 'Desde herramientas CLI en Rust hasta aplicaciones web full-stack con Django. Construimos exactamente lo que necesitas, ni más, ni menos.',
        'services.systems.title': 'Programación de Sistemas',
        'services.systems.desc': 'Infraestructura Unix/Linux, herramientas, automatización y optimización de rendimiento. El trabajo sin glamour que hace posible todo lo demás.',
        'services.ai.title': 'Integración de IA/ML',
        'services.ai.desc': 'Aplicaciones prácticas de modelos de lenguaje y análisis de datos. Sin exageraciones—soluciones reales que agregan valor a tus flujos de trabajo.',
        'services.it.heading': 'Soporte Técnico Local',
        'services.it.intro': 'Realizamos reparaciones selectivas y servicios de TI en sitio para clientes locales. Ya no somos un negocio físico con atención al público. Contáctenos para ver si su problema es uno que manejamos.',
        'services.it.repair': 'Reparación de Computadoras',
        'services.it.recovery': 'Recuperación de Datos',
        'services.it.business': 'TI Empresarial',
        'services.it.support': 'Soporte Técnico',
        'services.it.sales': 'Venta de Computadoras',
        'services.it.recycle': 'Reciclaje',
        'projects.title': 'Proyectos',
        'projects.intro': 'Herramientas y aplicaciones de código abierto que hemos construido. Código de calidad que puedes inspeccionar.',
        'contact.title': 'Contáctanos',
        'contact.intro': '¿Tienes un proyecto en mente? Hablemos de cómo podemos ayudarte.',
        'contact.name': 'Nombre *',
        'contact.email': 'Correo *',
        'contact.phone': 'Teléfono',
        'contact.message': 'Mensaje *',
        'contact.send': 'Enviar Mensaje',
        'contact.sending': 'Enviando...',
        'contact.chat': 'Chat en Vivo',
        'contact.coming': 'Próximamente',
        'contact.indev': 'En Desarrollo',
        'contact.location': 'Ubicación',
        'contact.address': 'Puerto Morelos, Riviera Maya<br>Entre Cancún y Playa Del Carmen<br>México',
        'contact.facebook': 'Escríbenos en Facebook',
        'contact.helpdesk.title': 'Mesa de Ayuda',
        'contact.helpdesk.desc': 'Consulta el estado de tu ticket de soporte',
        'contact.helpdesk.login': 'Iniciar Sesión',
        'footer.tagline': 'Consejos Honestos y Amigables Desde 2006',
        'footer.copyright': '© 2006–2026 South City Computer. Todos los derechos reservados.\nConstruido en Rust en South City Computer.',
        'footer.rating': 'de más de 200 reseñas',
        'footer.connect': 'Conectar',
        'footer.download': 'Descarga nuestra App',
        'projects.case.label': 'Caso de Estudio',
        'projects.case.title': 'El Sitio Web de 52ms',
        'projects.case.desc': 'Cómo construimos este sitio para cargar 46 veces más rápido que el promedio—usando Rust y WebP. Lee por qué la velocidad importa para tu negocio.',
        'projects.web.label': 'Diseño Web',
        'projects.web.title': 'Pet Friendly Vet',
        'projects.web.desc': 'Sitio web veterinario profesional generado desde contenido de Facebook usando IA. Rápido, móvil-amigable y optimizado para SEO.',
        'projects.viewall.title': 'Ver Todos los Proyectos',
        'projects.viewall.desc': 'Ve nuestro portafolio completo de trabajo para clientes y herramientas de código abierto.',
        'paradise.title': 'Encuéntrame en el Paraíso',
        'paradise.tagline': 'Programa unas vacaciones con el dinero que ahorrarás contratándonos. Planifica tu proyecto rodeado de belleza natural.',
        'paradise.intro': 'Como el turismo dental, pero para tecnología. Vuela a la Riviera Maya, reúnete cara a cara, y vete con un prototipo funcional—además de un bronceado. Estoy más relajado y productivo aquí, y esa energía se transfiere directamente a mejor trabajo para ti.',
        'paradise.packages': 'Paquetes de Retiro',
        'paradise.tech.title': 'Retiro Tecnológico',
        'paradise.tech.desc': 'Sesiones de prototipado rápido, planificación de arquitectura, desarrollo práctico. Vete con software funcional y documentación de implementación.',
        'paradise.mentor.title': 'Mentoría de Negocios',
        'paradise.mentor.desc': 'Sesiones de coaching estratégico con emprendedores experimentados. Desarrollo de modelo de negocio, posicionamiento de mercado, estrategia de crecimiento.',
        'paradise.story.title': 'Narrativa Documental',
        'paradise.story.desc': 'Aprende a contar la historia de tu marca a través de video. Coaching en estructura narrativa, narrativa visual y creación de contenido auténtico.',
        'paradise.wellness.title': 'Bienestar y Aventura',
        'paradise.wellness.desc': 'Yoga matutino, tours a cenotes, snorkel en el arrecife, exploración de la selva. Recarga mientras planificas—las mejores ideas llegan cuando estás relajado.',
        'paradise.cta': 'Combina consultoría con aventura caribeña. Trabajo remoto bienvenido. Retiros presenciales disponibles.',
        'paradise.button': 'Planifica Tu Retiro',
        'review.1': '"South City fue más allá de lo esperado. Trabajo en línea y dependo de mi laptop diariamente. Nestor arregló todos los problemas de urgencia y no perdí ninguna hora de trabajo. Es extremadamente conocedor y paciente con los novatos. ¡Si pudiera dar más de 5 estrellas, lo haría!"',
        'review.2': '"¡Son los mejores! Nunca he trabajado con una mejor empresa de tecnología. Diagnosticaron mi problema y lo arreglaron. Era complicado pero resolvió mi problema. Soy un pequeño negocio. Nestor es oficialmente nuestro nuevo departamento de TI."',
        'review.3': '"¡El personal aquí es súper servicial, considerado y excelente para trabajar! El dueño hace muchas de las reparaciones él mismo y hace un trabajo de calidad. ¡Pudo hacer que la laptop dañada por agua funcionara de nuevo!"',
        'review.4': '"Estos chicos siempre encuentran la manera de ayudarte. Nuestra computadora estaba completamente deshabilitada, aún así salvaron la información importante e incluso prestaron una computadora vieja para completar la presentación que debía entregarse ese día. ¡Salvaron el día!"',
        'review.5': '"Excelente servicio, rápido, completo. Katrina es una santa ya que me guió por el proceso para hacer accesibles los archivos transferidos. Recomendado para cualquiera que necesite servicio de computadoras de primera."',
        'review.6': '"¡Experiencia absolutamente increíble! Entré en pánico cuando mi laptop no encendía, justo en medio de la semana de exámenes finales. ¡Tuvieron mi laptop arreglada y lista en menos de 24 horas!"',
        'review.7': '"¡Estos chicos realmente saben lo que hacen! Son extremadamente conocedores y tienen un espacio deliciosamente peculiar que es divertido de visitar. Su sistema de tickets es fácil de usar y te mantiene actualizado."',
        'review.8': '"Llevé la computadora de mi esposa que estaba muy lenta y se congelaba. South City probó la computadora, respaldó todos los archivos, instaló un nuevo disco duro y reinstalaron los archivos rápidamente y a un gran precio."',
        'review.9': '"Excelente servicio...precios razonables...tomaron mi vieja laptop y la reconstruyeron a estado sólido..instalaron Linux..instalaron nuevos programas... ¡el mejor lugar para ir!"',
        'review.10': '"El sitio de South City Computer tenía un sistema de tickets fácil de usar, me respondió rápidamente, fue flexible en la programación y definitivamente mostró preocupación por la transparencia mientras me guiaba en cada paso."',
        'review.11': '"Mi hija dañó la pantalla de su laptop. La repararon como nueva. Una vez que llegó la pieza, la reparación se hizo ese mismo día. El cargo por el servicio pareció muy razonable."',
        'review.12': '"Excelente trabajo. Los he usado antes, incluso comprando una computadora reacondicionada de ellos hace varios años. El único lugar donde consideraría llevar mi computadora. No dudaría en recomendarlos."'
    }
};

let currentLang = localStorage.getItem('lang') || 'en';

function setLanguage(lang) {
    console.log('setLanguage called with:', lang);
    currentLang = lang;
    localStorage.setItem('lang', lang);

    // Check if we're on a blog page and need to redirect
    var currentPath = window.location.pathname;
    console.log('Current path:', currentPath);

    if (currentPath.includes('/blog/')) {
        console.log('On blog page, checking redirect...');
        var newPath = currentPath;
        var isSpanishPage = currentPath.includes('-es.html');

        if (lang === 'es' && !isSpanishPage) {
            // Switch to Spanish version
            newPath = currentPath.replace('.html', '-es.html');
            console.log('Switching to Spanish:', newPath);
        } else if (lang === 'en' && isSpanishPage) {
            // Switch to English version
            newPath = currentPath.replace('-es.html', '.html');
            console.log('Switching to English:', newPath);
        } else {
            console.log('Already on correct language version');
        }

        if (newPath !== currentPath) {
            console.log('Redirecting to:', newPath);
            window.location.href = newPath;
            return;
        }
    }

    document.querySelectorAll('[data-i18n]').forEach(function(el) {
        const key = el.getAttribute('data-i18n');
        if (translations[lang] && translations[lang][key]) {
            const text = translations[lang][key];
            if (text.includes('<br>')) {
                el.innerHTML = text;
            } else {
                el.textContent = text;
            }
        }
    });

    // Update active button state
    document.querySelectorAll('.lang-btn').forEach(function(btn) {
        btn.classList.remove('active');
        if (btn.getAttribute('data-lang') === lang) {
            btn.classList.add('active');
        }
    });

    // Update links with language-specific hrefs
    document.querySelectorAll('[data-i18n-href-es]').forEach(function(el) {
        var originalHref = el.getAttribute('data-original-href') || el.getAttribute('href');
        if (!el.getAttribute('data-original-href')) {
            el.setAttribute('data-original-href', originalHref);
        }
        if (lang === 'es') {
            el.setAttribute('href', el.getAttribute('data-i18n-href-es'));
        } else {
            el.setAttribute('href', originalHref);
        }
    });
}

// Make setLanguage globally accessible
window.setLanguage = setLanguage;

document.addEventListener('DOMContentLoaded', function() {
    // Initialize language
    setLanguage(currentLang);

    // Mobile Navigation Toggle - Simple click handler
    var navToggle = document.querySelector('.nav-toggle');
    var navLinks = document.querySelector('.nav-links');

    if (navToggle && navLinks) {
        navToggle.addEventListener('click', function(e) {
            e.stopPropagation();
            navLinks.classList.toggle('active');
        });

        // Close mobile menu when clicking a link
        navLinks.querySelectorAll('a').forEach(function(link) {
            link.addEventListener('click', function() {
                navLinks.classList.remove('active');
            });
        });

        // Close menu when clicking outside
        document.addEventListener('click', function(e) {
            if (!navToggle.contains(e.target) && !navLinks.contains(e.target)) {
                navLinks.classList.remove('active');
            }
        });
    }

    // Smooth scroll for anchor links
    document.querySelectorAll('a[href^="#"]').forEach(function(anchor) {
        anchor.addEventListener('click', function(e) {
            e.preventDefault();
            const targetId = this.getAttribute('href');
            const targetElement = document.querySelector(targetId);

            if (targetElement) {
                const navHeight = document.querySelector('.main-nav').offsetHeight;
                const targetPosition = targetElement.offsetTop - navHeight;

                window.scrollTo({
                    top: targetPosition,
                    behavior: 'smooth'
                });
            }
        });
    });

    // Contact Form Handling
    const contactForm = document.getElementById('contact-form');
    const formStatus = document.querySelector('.form-status');
    const submitBtn = document.querySelector('.btn-submit');

    if (contactForm) {
        contactForm.addEventListener('submit', async function(e) {
            e.preventDefault();

            // Reset status
            formStatus.className = 'form-status';
            formStatus.textContent = '';

            // Validate form
            const name = document.getElementById('name').value.trim();
            const email = document.getElementById('email').value.trim();
            const phone = document.getElementById('phone').value.trim();
            const message = document.getElementById('message').value.trim();

            if (!name || !email || !message) {
                showStatus('Please fill in all required fields.', 'error');
                return;
            }

            if (!isValidEmail(email)) {
                showStatus('Please enter a valid email address.', 'error');
                return;
            }

            // Show loading state
            submitBtn.classList.add('loading');
            submitBtn.disabled = true;

            try {
                const response = await fetch('/api/contact', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify({
                        name: name,
                        email: email,
                        phone: phone,
                        message: message
                    })
                });

                const data = await response.json();

                if (response.ok && data.success) {
                    if (data.view_url) {
                        showStatusWithLink(
                            'Thank you! Your message has been sent successfully.',
                            'View your submission',
                            data.view_url,
                            'success'
                        );
                    } else {
                        showStatus('Thank you! Your message has been sent successfully.', 'success');
                    }
                    contactForm.reset();
                } else {
                    showStatus(data.message || 'Something went wrong. Please try again.', 'error');
                }
            } catch (error) {
                // If the server isn't running, show a friendly message
                console.error('Contact form error:', error);
                showStatus('Unable to send message. Please try again later or reach out via GitHub.', 'error');
            } finally {
                submitBtn.classList.remove('loading');
                submitBtn.disabled = false;
            }
        });
    }

    function showStatus(message, type) {
        formStatus.textContent = message;
        formStatus.className = 'form-status ' + type;
    }

    function showStatusWithLink(message, linkText, linkUrl, type) {
        formStatus.innerHTML = message + ' <a href="' + linkUrl + '" target="_blank" class="view-link">' + linkText + '</a>';
        formStatus.className = 'form-status ' + type;
    }

    function isValidEmail(email) {
        const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
        return emailRegex.test(email);
    }

    // Navbar background on scroll
    const nav = document.querySelector('.main-nav');
    let lastScroll = 0;

    window.addEventListener('scroll', function() {
        const currentScroll = window.pageYOffset;

        if (currentScroll > 100) {
            nav.style.background = 'rgba(255, 255, 255, 0.98)';
        } else {
            nav.style.background = 'rgba(255, 255, 255, 0.95)';
        }

        lastScroll = currentScroll;
    });

    // Intersection Observer for scroll animations
    const observerOptions = {
        root: null,
        rootMargin: '0px',
        threshold: 0.1
    };

    const observer = new IntersectionObserver(function(entries) {
        entries.forEach(function(entry) {
            if (entry.isIntersecting) {
                entry.target.classList.add('visible');
            }
        });
    }, observerOptions);

    // Observe sections for animation
    document.querySelectorAll('section').forEach(function(section) {
        observer.observe(section);
    });

    // Lazy loading with preloading - load images before they enter viewport
    const lazyImages = document.querySelectorAll('img[data-src]');

    // Function to load an image
    function loadImage(img) {
        if (img.dataset.src) {
            img.src = img.dataset.src;
            img.removeAttribute('data-src');
            img.classList.add('loaded');
        }
    }

    // Load all images immediately - simpler and more reliable
    // IntersectionObserver was causing issues with images not triggering
    lazyImages.forEach(function(img) {
        loadImage(img);
    });

    // Load all background images immediately
    const parallaxSections = document.querySelectorAll('.location-banner');
    parallaxSections.forEach(function(section) {
        section.classList.add('bg-loaded');
    });

    // Service Intake Form Handling
    const intakeForms = document.querySelectorAll('.intake-form');

    intakeForms.forEach(function(form) {
        form.addEventListener('submit', async function(e) {
            e.preventDefault();

            const submitBtn = form.querySelector('button[type="submit"]');
            const formMessage = form.parentElement.querySelector('.form-message');

            // Get form data
            const formData = new FormData(form);
            const data = {};

            // Convert FormData to object, handling arrays (checkboxes)
            formData.forEach(function(value, key) {
                // Handle array-style keys like "problem[]"
                if (key.endsWith('[]')) {
                    const cleanKey = key.slice(0, -2);
                    if (!data[cleanKey]) {
                        data[cleanKey] = [];
                    }
                    data[cleanKey].push(value);
                } else {
                    data[key] = value;
                }
            });

            // Validate required fields
            if (!data.name || !data.email) {
                showIntakeMessage(formMessage, 'Please fill in name and email.', 'error');
                return;
            }

            if (!isValidEmail(data.email)) {
                showIntakeMessage(formMessage, 'Please enter a valid email address.', 'error');
                return;
            }

            // Show loading state
            submitBtn.disabled = true;
            submitBtn.textContent = 'Submitting...';

            try {
                const response = await fetch('/api/service-inquiry', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify(data)
                });

                const result = await response.json();

                if (response.ok && result.success) {
                    showIntakeMessage(formMessage, 'Thank you! We\'ll be in touch soon.', 'success');
                    form.reset();
                } else {
                    showIntakeMessage(formMessage, result.message || 'Something went wrong. Please try again.', 'error');
                }
            } catch (error) {
                console.error('Service inquiry error:', error);
                showIntakeMessage(formMessage, 'Unable to submit. Please try again later.', 'error');
            } finally {
                submitBtn.disabled = false;
                submitBtn.textContent = form.querySelector('button[type="submit"]').dataset.originalText || 'Submit Request';
            }
        });

        // Store original button text
        const btn = form.querySelector('button[type="submit"]');
        if (btn) {
            btn.dataset.originalText = btn.textContent;
        }
    });

    function showIntakeMessage(element, message, type) {
        if (element) {
            element.textContent = message;
            element.className = 'form-message ' + type;
            element.style.display = 'block';
        }
    }
});
