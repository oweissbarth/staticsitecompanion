CREATE TABLE `user` (
  `id` varchar(48) NOT NULL,
  `email` varchar(500) NOT NULL,
  PRIMARY KEY (`id`)
);

CREATE TABLE `download` (
  `id` varchar(48) NOT NULL,
  `name` varchar(255) NOT NULL,
  `user` varchar(48) NOT NULL,
  `download_count_offset` int(11) NOT NULL DEFAULT 0,
  PRIMARY KEY (`id`)
);


CREATE TABLE `download_log` (
  `id` varchar(48) NOT NULL,
  `download_version_id` varchar(48) NOT NULL,
  `datetime` datetime NOT NULL,
  `user_agent` varchar(500) DEFAULT NULL,
  `referrer` varchar(500) DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `downloadable` (`download_version_id`)
);

CREATE TABLE `download_version` (
  `id` varchar(48) NOT NULL,
  `version_number` varchar(50) NOT NULL,
  `resource_url` varchar(500) NOT NULL,
  `download_id` varchar(48) NOT NULL,
  `active` tinyint(1) NOT NULL,
  PRIMARY KEY (`id`)
);

CREATE TABLE `form` (
  `id` varchar(48) NOT NULL,
  `name` varchar(500) NOT NULL,
  `user` varchar(48) NOT NULL,
  `redirect_success` varchar(500) DEFAULT NULL,
  `redirect_failure` varchar(500) DEFAULT NULL,
  `notify_email` varchar(500) DEFAULT NULL,
  PRIMARY KEY (`id`)
);

CREATE TABLE `form_submission` (
  `id` varchar(48) NOT NULL,
  `form_id` varchar(48) NOT NULL,
  `content` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL CHECK (json_valid(`content`)),
  `user_agent` varchar(500) NOT NULL,
  `datetime` datetime NOT NULL,
  PRIMARY KEY (`id`)
);