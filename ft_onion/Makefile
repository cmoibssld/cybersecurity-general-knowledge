NAME = "ft_onion"
IMAGE_NAME = "onion_image"
SERVER_DIR = server/
HTML_DIR = html/

all: build run

build:
	docker build $(SERVER_DIR) --no-cache -t $(IMAGE_NAME)

run:
	docker run -p 8080:80 -d --name $(NAME) $(IMAGE_NAME) 
	
log:
	docker logs -f $(NAME)

down:
	docker stop $(NAME)

clean: down
	docker rm $(NAME)
	docker rmi $(IMAGE_NAME)

re: clean all
